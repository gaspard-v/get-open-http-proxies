use crate::callback::Generator;
use crate::tester::Proxy;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

pub struct ParallelProxies<'a, T: Generator> {
    generator: &'a mut T,
    tasks: JoinSet<()>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a, T: Generator + std::marker::Sync> ParallelProxies<'a, T> {
    const MAX_TASKS: usize = 100;
    pub fn new(generator: &'a mut T) -> Self {
        ParallelProxies {
            generator,
            tasks: JoinSet::new(),
        }
    }

    async fn process_proxy(address: String) {
        let mut proxy = Proxy::new(address.as_str()).unwrap();
        let response = proxy.send().await;
        match response {
            Ok(proxy) => {
                let status = proxy.get_status();
                println!("{}", status);
            }
            Err(error) => {
                eprintln!("Error occured: {}", error);
            }
        };
    }

    pub async fn get_all(&mut self) {
        let sem = Arc::new(Semaphore::new(Self::MAX_TASKS));

        while let Some(address) = self.generator.generate_address().await {
            let permit = Arc::clone(&sem).acquire_owned().await;
            self.tasks.spawn(async move {
                let _permit = permit;
                Self::process_proxy(address).await;
            });
        }
    }

    pub async fn pull_all(&mut self) {
        while let Some(task) = self.tasks.join_next().await {
            task.unwrap();
        }
    }
}
