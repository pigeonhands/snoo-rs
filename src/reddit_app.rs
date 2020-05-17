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

/// A 'connection' to reddit
/// Controlls how a `Reddit` instance communicates with the reddit http api.
#[derive(Clone)]
pub struct RedditApp {
    client: Client,
    pub(crate) rate_limiter: RateLimiter,
    auth: AuthType,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn reqwest_to_io_err(error: reqwest::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{}", error))
}

impl RedditApp {
    /// New app with no authenication and no rate limiter.
    pub fn new() -> io::Result<RedditApp> {
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
        Ok(RedditApp {
            client: client,
            rate_limiter: RateLimiter::Off,
            auth: AuthType::None,
        })
    }

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
        let grant = OAuthGrantType::Password{
            username,
            password
        };

        let auth = self.get_oauth_code(grant, id, Some(secret)).await?;
        self.authorize(auth).await
    }

    /// Autherise [RedditApp] as an application
    /// * `id` - reddit application id regstered on reddit
    /// * `redirect_url` - The app redirect url registered on reddit
    /// * `state` - state included in the redirect (can be anything)
    pub async fn authorize_application(
        &mut self,
        id: &str,
        scope: &[RedditAppScope],
        redirect_url: &str,
    ) -> io::Result<RedditAppAuthentication<'_>> {
        let scope_str = scope
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<&'static str>>()
            .join(",");

        use rand::{distributions::Alphanumeric, Rng};
        let state: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(7)
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
            ]).to_url();

        Ok(RedditAppAuthentication::new(self, id.to_owned(), redirect_url.as_str().to_owned(), state))
    }

    pub async fn me(&self) -> io::Result<OAuthMeResponse> {
        let target_url = endpoints::ME.oauth_ep()?.to_url();
        self.create_request::<OAuthMeResponse>(target_url).await
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
    pub async fn create_request<T: DeserializeOwned>(&self, target_url: Url) -> io::Result<T> {
        if self.rate_limiter.should_wait() {
            self.rate_limiter.wait().await;
        }

        let mut req = self.client.get(target_url);

        if let AuthType::OAuth(token) = &self.auth{
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
pub struct RedditAppAuthentication<'r> {
    app: &'r RedditApp,
    id: String,
    redirect_url: String,
    state: String,
    refresh_token: Option<String>,
}

impl<'r> RedditAppAuthentication<'r> {
    pub fn new(app: &'r RedditApp, id: String, redirect_url: String, state: String) -> Self {
        Self {
            app,
            id,
            redirect_url,
            state,
            refresh_token: None,
        }
    }

    // todo: add authentication for applciation
    pub async fn authenticate(&self, code: &str) -> io::Result<()> {
        let grant = OAuthGrantType::AutherizationCode {
            code,
            redirect_url: &self.redirect_url
        }; 

        let auth = self.app.get_oauth_code(grant, &self.id, None).await?;
        //self.app.authorize(auth).await?;

        Ok(())
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

pub enum RedditAppScope {
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

impl RedditAppScope {
    pub fn all() -> Vec<RedditAppScope> {
        vec![
            RedditAppScope::Identity,
            RedditAppScope::Edit,
            RedditAppScope::Flair,
            RedditAppScope::History,
            RedditAppScope::PrivateMessages,
            RedditAppScope::Read,
            RedditAppScope::Report,
            RedditAppScope::Save,
            RedditAppScope::Submit,
            RedditAppScope::Subscribe,
            RedditAppScope::Vote,
            RedditAppScope::WikiEdit,
            RedditAppScope::WikiRead,
            RedditAppScope::Account,
            RedditAppScope::MySubreddits,
            RedditAppScope::ModConfig,
            RedditAppScope::ModFlair,
            RedditAppScope::ModLog,
            RedditAppScope::ModPost,
            RedditAppScope::ModWiki,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RedditAppScope::Identity => "identity",
            RedditAppScope::Edit => "edit",
            RedditAppScope::Flair => "flair",
            RedditAppScope::History => "history",
            RedditAppScope::PrivateMessages => "privatemessages",
            RedditAppScope::Read => "read",
            RedditAppScope::Report => "report",
            RedditAppScope::Save => "save",
            RedditAppScope::Submit => "submit",
            RedditAppScope::Subscribe => "subscribe",
            RedditAppScope::Vote => "vote",
            RedditAppScope::WikiEdit => "wikiedit",
            RedditAppScope::WikiRead => "wikiread",
            RedditAppScope::Account => "account",
            RedditAppScope::MySubreddits => "mysubreddits",
            RedditAppScope::ModConfig => "modconfig",
            RedditAppScope::ModFlair => "modflair",
            RedditAppScope::ModLog => "modlog",
            RedditAppScope::ModPost => "modposts",
            RedditAppScope::ModWiki => "modwiki",
        }
    }
}
