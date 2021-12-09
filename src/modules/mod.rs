use async_trait::async_trait;
use reqwest::Client;

use crate::Error;

mod http;
mod subdomains;

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubDomainModule: Module {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}

#[async_trait]
pub trait HttpModule: Module {
    async fn scan(&self, http_client: &Client, endpoint: &str) -> Result<Option<HttpFinding>, Error>;
}

#[derive(Debug, Clone)]
pub enum HttpFinding {
}