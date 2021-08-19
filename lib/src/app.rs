use std::fmt;

use cfg::NetworkConfig;
use db::Registry;

pub struct Client {
  engine: net::Client,
  registry: Registry,
  config: NetworkConfig,
}

impl Client {
  pub fn get(&self, key: String) -> net::Result<String> {
    Ok("heythere".to_string())
  }
}
pub struct Server {
  engine: net::Server,
  registry: Registry,
  config: NetworkConfig,
}

impl Server {}
