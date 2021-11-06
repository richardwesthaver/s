//! bin/shc.rs --- shed-cli
use rlib::util::Result;
use shed::{build_cli, App};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  // collect args
  let cli = build_cli().version(env!("DEMON_VERSION")).get_matches(); 

  let app = App::new(&cli)?; //initialize
  app.dispatch().await?; //dispatch

  Ok(())
}
