//! Creates requests to the reddit api with the specified
//! rate limiting and authentication
use crate::rate_limit::{RateLimiter, RateLimiterTracker};

use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;
use std::io;

use crate::endpoints::{self, Endpoint, EndpointBase, EndpointBuilder};

use crate::models::auth::{AuthResponse, OAuthMeResponse};

#[derive(Clone)]
enum AuthType {
    OAuth(String),
    None,
}

/// A connection to the reddit api.
#[derive(Clone)]
pub struct RedditApi {
    client: Client,
    pub(crate) rate_limiter: RateLimiter,
    auth: AuthType,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn reqwest_to_io_err(error: reqwest::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{}", error))
}

impl RedditApi {
    /// New app with no authenication and no rate limiter.
    pub fn new() -> io::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .build()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Interrupted,
                    format!("Failed to create http client. {:?}", e),
                )
            })?;
        Ok(Self {
            client: client,
            rate_limiter: RateLimiter::Off,
            auth: AuthType::None,
        })
    }

    /// Request a new oauth code from the reddit api.
    pub async fn get_oauth_code(
        &self,
        grant_type: OAuthGrantType<'_>,
        id: &str,
        secret: Option<&str>,
    ) -> io::Result<AuthResponse> {
        let req = self
            .client
            .post(endpoints::ACCESS_TOKEN.ssl_ep()?.to_url())
            .form(&grant_type.as_params())
            .basic_auth(id, secret);

        let resp = req
            .send()
            .await
            .map_err(reqwest_to_io_err)?
            .json::<AuthResponse>()
            .await
            .map_err(reqwest_to_io_err)?;

        Ok(resp)
    }

    /// Creates a new authenicated script application
    async fn authorize(&mut self, auth: AuthResponse) -> io::Result<()> {
        if let Some(err) = auth.error {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error was retuened. {}", err),
            ))?;
        }

        let token = auth.access_token.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "Failed to get access token.",
            )
        })?;

        self.auth = AuthType::OAuth(token);
        self.rate_limiter = RateLimiter::new_batched();
        Ok(())
    }

    pub async fn authorize_script(
        &mut self,
        username: &str,
        password: &str,
        id: &str,
        secret: &str,
    ) -> io::Result<()> {
        let grant = OAuthGrantType::Password { username, password };

        let auth = self.get_oauth_code(grant, id, Some(secret)).await?;
        self.authorize(auth).await
    }

    /// Autherise [RedditApi] as an application
    /// * `id` - reddit application id regstered on reddit
    /// * `auth_url` - The generated authentication callback url (can be generated with [RedditApi::create_authorization_url])
    pub async fn authorize_application(
        &mut self,
        code: &str,
        auth_url: &RedditApiAuthenticationUrl,
    ) -> io::Result<()> {
        let grant = OAuthGrantType::AutherizationCode {
            code,
            redirect_url: auth_url.redirect_url(),
        };

        let auth = self.get_oauth_code(grant, auth_url.id(), None).await?;
        self.authorize(auth).await
    }

    /// Creates a new authenticaton url with a
    /// given scope and random state.
    pub async fn create_authorization_url(
        &mut self,
        id: &str,
        scope: &[RedditApiScope],
        redirect_url: &str,
    ) -> io::Result<RedditApiAuthenticationUrl> {
        let scope_str = scope
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<&'static str>>()
            .join(",");

        use rand::{distributions::Alphanumeric, Rng};
        let state: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(5)
            .collect();

        let redirect_url = endpoints::AUTHERIZE_APPLICATION
            .ssl_ep()?
            .add_query_pairs(&[
                ("response_type", "code"),
                ("duration", "permanent"),
                ("client_id", id),
                ("state", &state),
                ("redirect_uri", redirect_url),
                ("scope", &scope_str),
            ])
            .to_url();

        Ok(RedditApiAuthenticationUrl::new(
            id.to_owned(),
            redirect_url.as_str().to_owned(),
            Some(state),
        ))
    }

    pub async fn me(&self) -> io::Result<OAuthMeResponse> {
        let target_url = endpoints::ME.oauth_ep()?.to_url();
        self.get_api::<OAuthMeResponse>(target_url).await
    }

    /// Creates a new endpoint with the corrent base depending on
    /// the authentication state of the application.
    /// e.g.
    /// No authenticaion => www.reddit,
    /// Authenitcated => oauth.reddit
    pub fn create_endpoint(&self, builder: EndpointBuilder) -> io::Result<Endpoint> {
        let ep_base = match self.auth {
            AuthType::None => EndpointBase::Regular,
            _ => EndpointBase::OAuth,
        };
        Endpoint::new(ep_base, builder)
    }

    /// Create an endpoint from a string
    /// Same as ```create_endpoint(Endpoint::build("my-endpoint")```
    pub fn create_endpoint_str(&self, str_ep: &str) -> io::Result<Endpoint> {
        self.create_endpoint(Endpoint::build(str_ep))
    }

    /// Validates status code and updates the
    // rate limiter if enabled.
    fn handle_http_response(&self, resp: &Response) -> io::Result<()> {
        if !resp.status().is_success() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("A non-success http response was retuned: {}", resp.status()),
            ))?;
        }

        // TODO: Make less ugly.
        if self.rate_limiter.should_update() {
            let headers = resp.headers();
            let read_header = |h| {
                let mut out = None;
                if let Some(h) = headers.get(h) {
                    if let Ok(val) = h.to_str() {
                        out = Some(val.parse::<f32>().unwrap_or(0.0) as i32);
                    }
                }
                out
            };

            let get_tracker = || {
                let tracker = RateLimiterTracker::from_values(
                    read_header(RateLimiter::REMANING_HEADER)?,
                    read_header(RateLimiter::USED_HEADER)?,
                    read_header(RateLimiter::RESET_HEADER)?,
                );
                Some(tracker)
            };

            if let Some(tracker) = get_tracker() {
                self.rate_limiter.update(tracker);
            }
        }

        Ok(())
    }

    /// Creates a GET request to an endpoint with
    /// the applications rate limiter and session/cookies/auth.
    pub async fn get_api<T: DeserializeOwned>(&self, target_url: Url) -> io::Result<T> {
        if self.rate_limiter.should_wait() {
            self.rate_limiter.wait().await;
        }

        let mut req = self.client.get(target_url);

        if let AuthType::OAuth(token) = &self.auth {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("Failed to send get request. {}", e),
            )
        })?;

        self.handle_http_response(&resp)?;

        let data = resp.json::<T>().await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("Failed to deseralize response. {}", e),
            )
        })?;

        Ok(data)
    }
}

/// Listens on authenticationc callback url for a
/// autentication code
pub struct RedditApiAuthenticationUrl {
    id: String,
    redirect_url: String,
    state: Option<String>,
}

impl RedditApiAuthenticationUrl {
    pub fn new(id: String, redirect_url: String, state: Option<String>) -> Self {
        Self {
            id,
            redirect_url,
            state,
        }
    }

    pub fn redirect_url(&self) -> &str {
        &self.redirect_url
    }

    pub fn state(&self) -> Option<&str> {
        Some(self.state.as_ref()?)
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub enum OAuthGrantType<'a> {
    Password {
        username: &'a str,
        password: &'a str,
    },
    AutherizationCode {
        code: &'a str,
        redirect_url: &'a str,
    },
}

impl<'r> OAuthGrantType<'r> {
    pub fn as_params(&self) -> Vec<(&'r str, &'r str)> {
        match self {
            OAuthGrantType::Password { username, password } => vec![
                ("grant_type", "password"),
                ("username", username),
                ("password", password),
            ],
            OAuthGrantType::AutherizationCode { code, redirect_url } => vec![
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", redirect_url),
            ],
        }
    }
}

pub enum RedditApiScope {
    Identity,
    Edit,
    Flair,
    History,
    PrivateMessages,
    Read,
    Report,
    Save,
    Submit,
    Subscribe,
    Vote,
    WikiEdit,
    WikiRead,
    Account,
    MySubreddits,
    ModConfig,
    ModFlair,
    ModLog,
    ModPost,
    ModWiki,
}

impl RedditApiScope {
    pub fn all() -> Vec<RedditApiScope> {
        vec![
            RedditApiScope::Identity,
            RedditApiScope::Edit,
            RedditApiScope::Flair,
            RedditApiScope::History,
            RedditApiScope::PrivateMessages,
            RedditApiScope::Read,
            RedditApiScope::Report,
            RedditApiScope::Save,
            RedditApiScope::Submit,
            RedditApiScope::Subscribe,
            RedditApiScope::Vote,
            RedditApiScope::WikiEdit,
            RedditApiScope::WikiRead,
            RedditApiScope::Account,
            RedditApiScope::MySubreddits,
            RedditApiScope::ModConfig,
            RedditApiScope::ModFlair,
            RedditApiScope::ModLog,
            RedditApiScope::ModPost,
            RedditApiScope::ModWiki,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RedditApiScope::Identity => "identity",
            RedditApiScope::Edit => "edit",
            RedditApiScope::Flair => "flair",
            RedditApiScope::History => "history",
            RedditApiScope::PrivateMessages => "privatemessages",
            RedditApiScope::Read => "read",
            RedditApiScope::Report => "report",
            RedditApiScope::Save => "save",
            RedditApiScope::Submit => "submit",
            RedditApiScope::Subscribe => "subscribe",
            RedditApiScope::Vote => "vote",
            RedditApiScope::WikiEdit => "wikiedit",
            RedditApiScope::WikiRead => "wikiread",
            RedditApiScope::Account => "account",
            RedditApiScope::MySubreddits => "mysubreddits",
            RedditApiScope::ModConfig => "modconfig",
            RedditApiScope::ModFlair => "modflair",
            RedditApiScope::ModLog => "modlog",
            RedditApiScope::ModPost => "modposts",
            RedditApiScope::ModWiki => "modwiki",
        }
    }
}
