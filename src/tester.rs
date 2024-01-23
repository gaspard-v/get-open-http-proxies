use reqwest::{self, Error};

pub struct Proxy<'a> {
    proxy_url: &'a str,
    target_url: &'a str,
    client: reqwest::Client,
    response: Option<reqwest::Response>,
}

impl<'a> Proxy<'a> {
    pub fn new(proxy_url: &'a str) -> Result<Self, Error> {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        let timeout = std::time::Duration::new(10, 0);
        let client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(timeout)
            .build()?;
        Ok(Proxy {
            proxy_url,
            target_url: "https://httpbin.org/ip",
            client,
            response: None,
        })
    }

    pub async fn send(&self) -> Result<reqwest::StatusCode, Error> {
        let response = self.client.get(self.target_url).send().await?;
        Ok(response.status())
    }
}
