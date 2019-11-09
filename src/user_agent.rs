use reqwest::header;

use super::constants::{APIKEY, ENDPOINT, NAME, VERSION};

pub fn user_agent() -> String {
    format!("{}/{}", NAME, VERSION)
}

pub fn construct_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Apikey {}", APIKEY.as_str()).as_str()).unwrap(),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        header::HeaderValue::from_str(user_agent().as_str()).unwrap(),
    );
    headers
}

pub fn get_client(route: &str) -> reqwest::RequestBuilder {
    let url = format!("{}{}", ENDPOINT.as_str(), route);
    let client = reqwest::Client::new();
    client.get(url.as_str()).headers(construct_headers())
}
