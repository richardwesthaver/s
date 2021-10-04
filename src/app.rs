use crate::Config;
use rlib::{db::{
    Error as DbErr,
    registry::Registry,
  }, kala::{
    Error as KErr,
    cmd::{
      hg::{hgweb, hg}, shell::make,
    }
  }, logger::log::error, net::{Client, Error as NetErr, reqwest::{self, Url}, ssh}, util::Result};

use std::{fs::File, path::{PathBuf, Path}, str::FromStr};
pub struct App {
  pub cfg: Config,
}

impl App {
  pub fn new(cfg: Config) -> Self {
    let shed_path: PathBuf = cfg.path.clone();
    match shed_path.join("data/log/shed").to_str() {
      Some(p) => {rlib::logger::file("debug", p, "shed").expect("logger init failed");},
      None => rlib::logger::flexi("info").expect("logger init failed")
    };

    App {cfg}
  }

  pub fn init<P: AsRef<Path>>(&self, path: P, fmt: Option<&str>) -> Result<()> {
    let p = path.as_ref();
    println!("initializing {}...", &p.display());
    match fmt {
      Some("ron") | None => self.cfg.write(&p,None)?,
      Some("json") => self.cfg.write(&p, Some("json"))?,
      Some("bin") => unimplemented!(),
      Some(_) => error!("unknown configuration type"),
    }
    Ok(())
  }

  pub async fn build(&mut self, target: &str, pkg: &str, ) -> Result<(), KErr> {
    if self.cfg.src.drain_filter(|src| src.name != *pkg).count() > 0 {
      println!("matched packages");
    };
    make(vec![target]).await?;
    Ok(())
  }

  pub fn db_init(&self) -> Result<Registry, DbErr> {
    let shed_path: PathBuf = self.cfg.path.clone();
    Registry::new(shed_path.join("data/db"))
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

  pub async fn request(&self, t: &str, resource: &str) -> Result<(), NetErr> {
    let cfg = self.cfg.network.clone();
    let _client = Client { cfg };
    let dst = self.cfg.path.join("stash/tmp/").join(&resource);
    match t {
      "hg" => {
        println!("requesting mercurial repo: {}", resource);
        let u = format!("https://hg.rwest.io/{}", &resource);
        hg(vec!["clone", &u, dst.to_str().unwrap()]).await; // this should be fallible
        println!("repo created at {}", dst.display());
      },
      "dm" => println!("sending message to: {}", resource),
      "stash" => {
        println!("requesting resource: {}", resource);
        let u = format!("https://cdn.rwest.io/{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      },
      "store" => {
        println!("requesting resource: {}", resource);
        let u = format!("https://pkg.rwest.io/{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      },
      "http" => {
        println!("requesting resource over http: {}", &resource);
        download(Url::from_str(&resource).unwrap(), &dst).await.unwrap();
      }
      "ssh" => println!("requesting resource over ssh: {}", resource),
      _ => error!("unrecognized server type {:?}", t),
    }
    Ok(())
  }
}

async fn download<P: AsRef<Path>>(url: reqwest::Url, path: P) -> Result<(), NetErr> {
  let res = reqwest::get(url).await?;
  let mut dst = {
    let fname = res.url().path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() {None} else {Some(name)})
      .expect("could not create path for url");
    let fname = path.as_ref().join(fname);
    println!("downloading file to {}", fname.display());
    File::create(fname)?
  };
  let content = res.text().await?;
  std::io::copy(&mut content.as_bytes(), &mut dst)?;
  Ok(())
}
