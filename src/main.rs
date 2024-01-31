#![warn(rust_2018_idioms)]
mod callback;
mod parallel;
mod tester;
use crate::callback::Generator;
use crate::parallel::ParallelProxies;

struct IpGenerator {
    max_iteration: usize,
    current_iteration: usize,
}

impl IpGenerator {
    fn new(iteration: usize) -> Self {
        IpGenerator {
            max_iteration: iteration,
            current_iteration: 0,
        }
    }
}

impl Generator for IpGenerator {
    async fn generate_address(&mut self) -> Option<String> {
        if self.current_iteration >= self.max_iteration {
            return None;
        }
        self.current_iteration += 1;
        Some(String::from("http://127.0.0.1:3128"))
    }
}

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
