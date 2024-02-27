#![warn(rust_2018_idioms)]
mod callback;
mod examples;
mod parallel;
mod tester;
use crate::examples::ip::IpGenerator;
use crate::parallel::ParallelProxies;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut generator = IpGenerator::new(200);
    let mut parallel = ParallelProxies::new(&mut generator);
    parallel.get_all().await;
    parallel.pull_all().await;
    Ok(())
}
