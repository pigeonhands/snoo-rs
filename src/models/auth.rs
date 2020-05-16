use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub error: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i32>,
    pub scope: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OAuthMeResponse {
    pub comment_karma: i32,
    pub link_karma: i32,
    pub created: f64,
    pub created_utc: f64,
    pub has_mail: bool,
    pub has_mod_mail: bool,
    pub has_verified_email: bool,
    pub id: String,
    pub name: String,
    pub is_gold: bool,
    pub is_mod: bool,
    pub over_18: bool,
}
