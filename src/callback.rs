pub trait Generator {
    async fn generate_address(&mut self) -> Option<String>;
}
