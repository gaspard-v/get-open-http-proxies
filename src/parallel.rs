use crate::tester::Proxy;
use tokio::task;
use tokio::task::JoinSet;

type Callback = fn() -> Option<String>;

pub struct ParallelProxies {
    address_generator: Callback,
    tasks: JoinSet<()>,
}

impl ParallelProxies {
    const MAX_TASKS: i32 = 100;
    pub fn new(&mut self, address_generator: Callback) -> Self {
        ParallelProxies {
            address_generator,
            tasks: JoinSet::new(),
        }
    }
    async fn process_proxy(address: String) {
        let mut proxy = Proxy::new(address.as_str()).unwrap();

        // TODO: change "unwrap" and handle the error
        proxy.send().await.unwrap();
        let status = proxy.get_status();
    }
    pub fn get_all(&mut self) -> i32 {
        let mut i = 0;
        while let Some(address) = (self.address_generator)() {
            if Self::MAX_TASKS <= i {
                break;
            }
            self.tasks.spawn(async move {
                Self::process_proxy(address).await;
            });
            i += 1;
        }
        i
    }
}
