/*
 * Jupiter APIv4
 *
 * Jupiter quote and swap API
 *
 * The version of the OpenAPI document: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

pub const DEFAULT_BASE_URL: &str = "https://quote-api.jup.ag";

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: DEFAULT_BASE_URL.to_owned(),
            user_agent: Some("JupiterAPI/OpenAPI/0.0.1/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}

impl Configuration {
    pub fn new(base_path: &'static str) -> Self {
        Configuration {
            base_path: base_path.to_owned(),
            ..Default::default()
        }
    }
}
