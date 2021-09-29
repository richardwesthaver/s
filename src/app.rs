use crate::Config;
use rlib::{
  db::registry::Registry,
  kala::cmd::hg::hgweb,
  logger::log::error,
  net::{
    Client,
    Result,
    /*    engine::{
     *      dns::{self, resolver::Lookup},
     *      http::{service, Router, ServeDir} */
  },
};

use std::path::PathBuf;

pub struct App {
  pub cfg: Config,
  pub registry: Registry,
}

impl App {
  pub fn new(cfg: Config) -> Self {
    let shed_path: PathBuf = cfg.path.clone();
    let log_path = shed_path.join("data/log");
    let log_name = "shed";
    rlib::logger::file("info, rlib = trace", log_path.to_str().unwrap(), log_name).unwrap();
    App {
      cfg,
      registry: Registry::new(shed_path.join("data/db")).unwrap(),
    }
  }

  pub fn init(&self, path: &str, fmt: Option<&str>) -> Result<()> {
    println!("initializing shed...");
    match fmt {
      Some("ron") | None => self.cfg.write(path,None).unwrap(),
      Some("json") => self.cfg.write(path, Some("json")).unwrap(),
      Some("bin") => unimplemented!(),
      Some(_) => error!("unknown configuration type"),
    }
    Ok(())
  }

  pub async fn serve(&self, engine: &str) -> Result<()> {
    match engine {
      "hg" => hgweb(&self.cfg.hgrc)
        .await
        .expect("encountered error in hg_serve process"),
      "dm" => {
        println!("waiting for dm...")
      }
      _ => {
        error!("unrecognized server type!")
      }
    }
    Ok(())
  }

  pub async fn request(&self, ty: &str, resource: &str) -> Result<()> {
    let cfg = self.cfg.network.clone();
    let _client = Client { cfg };
    match ty {
      "hg" => println!("requesting mercurial repo: {}", resource),
      "dm" => println!("sending message to: {}", resource),
      "stash" => println!("requesting resource: {}", resource),
      "store" => println!("requesting resource: {}", resource),
      "http" => {
        println!("requesting resource over http: {}", &resource);
      }
      "ssh" => println!("requesting resource over ssh: {}", resource),
      _ => error!("unrecognized server type {:?}", ty),
    }
    Ok(())
  }
}
