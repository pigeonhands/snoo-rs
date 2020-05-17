use crate::rate_limit::{RateLimiter, RateLimiterTracker};

use reqwest::{Client, Response, Url};
use serde::{de::DeserializeOwned, Deserialize};
use std::io;

use crate::endpoints::{Endpoint, EndpointBase, EndpointBuilder};

use crate::models::auth::{AuthResponse, OAuthMeResponse};

#[derive(Clone)]
enum AuthType {
    Script(String),
    InstalledApp,
    None,
}

#[derive(Clone)]
pub struct RedditApp {
    client: Client,
    pub(crate) rate_limiter: RateLimiter,
    auth: AuthType,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl RedditApp {
    const ACCESS_TOKEN_ENDPOINT: &'static str = "https://ssl.reddit.com/api/v1/access_token/.json";
    const OAUTH_ME_ENDPOINT: &'static str = "https://oauth.reddit.com/api/v1/me";

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

    pub async fn new_script(
        username: &str,
        password: &str,
        id: &str,
        secret: &str,
    ) -> io::Result<Self> {
        let mut app = Self::new()?;

        let params = [
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let req = app
            .client
            .post(Self::ACCESS_TOKEN_ENDPOINT)
            .form(&params)
            .basic_auth(id, Some(secret));

        let resp = req
            .send()
            .await
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    format!("Request to authenticate failed.: {}", e),
                )
            })?
            .json::<AuthResponse>()
            .await
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to deseralize json response. {}", e),
                )
            })?;

        if let Some(err) = resp.error {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error was retuened. {}", err),
            ))?;
        }

        let token = resp.access_token.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "Failed to get access token.",
            )
        })?;

        app.auth = AuthType::Script(token);
        app.rate_limiter = RateLimiter::new_batched();
        Ok(app)
    }

    pub async fn me(&self) -> io::Result<OAuthMeResponse> {
        let endpoint = Url::parse(Self::OAUTH_ME_ENDPOINT)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "err"))?;

        self.create_request::<OAuthMeResponse>(endpoint).await
    }

    /// Builds a new ep
    pub fn create_endpoint(&self, builder: EndpointBuilder) -> io::Result<Endpoint> {
        let ep_base = match self.auth {
            AuthType::None => EndpointBase::Regular,
            _ => EndpointBase::OAuth,
        };
        Endpoint::new(ep_base, builder)
    }

    /// Builds a new ep from a string
    pub fn create_enddpoint_str(&self, str_ep: &str) -> io::Result<Endpoint> {
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
            } else {
                println!("No tracker.");
            }
        }

        Ok(())
    }
    /// Used for all [GET] api calls.
    pub async fn create_request<T: DeserializeOwned>(&self, target_url: Url) -> io::Result<T> {
        if self.rate_limiter.should_wait() {
            self.rate_limiter.wait().await;
        }

        let mut req = self.client.get(target_url);

        match &self.auth {
            AuthType::Script(token) => req = req.bearer_auth(token),
            _ => {}
        };

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
