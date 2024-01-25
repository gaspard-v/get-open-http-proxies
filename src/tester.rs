use reqwest::{self, Error};

pub struct Proxy<'a> {
    proxy_url: &'a str,
    target_url: &'a str,
    client: reqwest::Client,
    response: Option<reqwest::Response>,
}

impl<'a> Proxy<'a> {
    const DEFAULT_TARGET_URL: &'static str = "https://httpbin.org/ip";
    pub fn new(proxy_url: &'a str) -> Result<Self, Error> {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        let timeout = std::time::Duration::new(10, 0);
        let client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(timeout)
            .build()?;
        Ok(Proxy {
            proxy_url,
            target_url: Self::DEFAULT_TARGET_URL,
            client,
            response: None,
        })
    }

    pub async fn send(&mut self) -> Result<&Self, Error> {
        self.response = Some(self.client.get(self.target_url).send().await?);
        Ok(self)
    }

    pub fn get_proxy_url(&self) -> &str {
        self.proxy_url
    }

    pub fn get_status(&self) -> reqwest::StatusCode {
        self.response.as_ref().unwrap().status()
    }

    pub async fn get_text(self) -> Result<String, Error> {
        self.response.unwrap().text().await
    }

    pub fn set_target_url(&mut self, target_url: &'a str) -> &Self {
        self.target_url = target_url;
        self
    }
}
