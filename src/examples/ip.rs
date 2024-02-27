use crate::callback::Generator;

pub struct IpGenerator {
    max_iteration: usize,
    current_iteration: usize,
}

impl IpGenerator {
    pub fn new(iteration: usize) -> Self {
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
