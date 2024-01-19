#![warn(rust_2018_idioms)]
#![deny(warnings)]

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let timeout = std::time::Duration::new(10, 0);
    let proxy_url = "http://127.0.0.1:3128";
    let target_url = "https://httpbin.org/ip";
    let proxy = reqwest::Proxy::all(proxy_url)?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .timeout(timeout)
        .build()?;
    let response = client.get(target_url).send().await?;
    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body: {}", body);
    Ok(())
}
