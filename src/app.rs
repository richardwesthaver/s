use crate::Config;
use kala::cmd::hg::hgweb;
use net::{Client, Server};

pub struct App {
  pub cfg: Config,
}

impl App {
  pub fn new(cfg: Config) -> Self {
    App {cfg}
  }

  pub fn serve(&self, ty: String) -> net::Result<()> {
    let cfg = self.cfg.network.clone();
    let _server = Server { cfg };
    match ty.as_str() {
      "hg" => {
        println!("starting hgweb...");
        hgweb(&self.cfg.hgrc)?;
      }
      "dm" => {
        println!("waiting for dm...")
      }
      _ => {
        eprintln!("unrecognized server type {:?}", ty)
      }
    }
    Ok(())
  }

  pub fn request(&self, ty: String, resource: String) -> net::Result<()> {
    let cfg = self.cfg.network.clone();
    let _client = Client { cfg };
    match ty.as_str() {
      "hg" => println!("requesting mercurial repo: {}", resource),
      "dm" => println!("sending message to: {}", resource),
      _ => eprintln!("unrecognized server type {:?}", ty),
    }
    Ok(())
  }

}
