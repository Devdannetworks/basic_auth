use anyhow::{Error, Result};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/get";
    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client.get(url).basic_auth(user, passwd).send().await?;

    println!("Status: {}", response.status());
    println!("Headers: \n{:?}", response.headers());
    let body = response.text().await?;
    println!("Body: \n{}", body);

    Ok(())
}
