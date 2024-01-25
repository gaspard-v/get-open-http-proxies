#![warn(rust_2018_idioms)]
mod parallel;
mod tester;
use crate::tester::Proxy;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let status = Proxy::new("http://127.0.0.1:3128")?
        .send()
        .await?
        .get_status();
    println!("status: {}", status);
    Ok(())
}
