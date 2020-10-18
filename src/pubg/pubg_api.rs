use reqwest::{header,Result};
use serde_json::{Value};
pub async fn get(endpoint: &str) -> Result<Value> {

    const KEY: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI0YmQ2ZTJmMC1jM2M1LTAxMzgtOTQ0ZS0xOTdlNDVlMjM0OWUiLCJpc3MiOiJnYW1lbG9ja2VyIiwiaWF0IjoxNTk3Nzg1MDg0LCJwdWIiOiJibHVlaG9sZSIsInRpdGxlIjoicHViZyIsImFwcCI6ImN3YXRzb24xOTg4LWdtIn0.Mt8A76L-gEWUvCpcrYAo4Wl1dS0sA23oKZjhdEJSqfA";

    const BASE_URL: &str = "https://api.pubg.com";

    let url = format!("{}{}", BASE_URL, endpoint);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("bearer"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.api+json"),
    );

    // get a client builder
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client
        .get(&url)
        .header(header::AUTHORIZATION, KEY) // used to pass the key
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
