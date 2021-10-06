use crate::Config;
use rlib::{
  db::{registry::Registry, Error as DbErr},
  kala::{
    cmd::{
      hg::{hg, hgweb},
      shell::make,
    },
    Error as KErr,
  },
  logger::log::{error, debug},
  net::{
    reqwest::{self, Url},
    Client, Error as NetErr,
  },
  obj::config::{Oauth2Config, SshConfig},
  util::Result,
};

use std::{
  fs::File,
  path::{Path, PathBuf},
  str::FromStr,
};
pub struct App {
  pub cfg: Config,
}

impl App {
  pub fn new(cfg: Config) -> Self {
    let shed_path: PathBuf = cfg.path.clone();
    debug!("App Config: {:?}", cfg);
    match shed_path.join("data/log").to_str() {
      Some(p) => {
        rlib::logger::file("rlib=debug,trace", p, "shed").expect("logger init failed");
      }
      None => rlib::logger::flexi("info").expect("logger init failed"),
    };
    App { cfg }
  }

  pub fn init<P: AsRef<Path>>(&self, path: P, fmt: Option<&str>) -> Result<()> {
    let p = path.as_ref();
    println!("initializing {}...", &p.display());
    match fmt {
      Some("ron") | None => self.cfg.write(&p, None)?,
      Some("json") => self.cfg.write(&p, Some("json"))?,
      Some("bin") => self.cfg.write(&p, Some("bin"))?,
      Some(_) => error!("unknown configuration type"),
    }
    Ok(())
  }

  pub async fn build(&mut self, target: &str, pkg: &str) -> Result<(), KErr> {
    if self.cfg.src.drain_filter(|src| src.name != *pkg).count() > 0 {
      println!("matched packages");
    };
    make(vec![target]).await?;
    Ok(())
  }

  pub fn db_init(&self) -> Result<(), DbErr> {
    let db_path: PathBuf = self.cfg.path.clone().join("data/db");
    std::fs::remove_dir_all(&db_path)?;
    Registry::new(&db_path)?;
    Ok(())
  }

  pub async fn serve(&self, engine: &str) -> Result<()> {
    match engine {
      "hg" => hgweb(&self.cfg.hg)
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

  pub async fn dl(&self, t: &str, resource: &str) -> Result<(), NetErr> {
    let cfg = self.cfg.network.clone();
    let _client = Client { cfg };
    let dst = self.cfg.path.join("stash/tmp/");
    match t {
      "hg" => {
        let u = format!("https://hg.rwest.io/{}", &resource);
        if resource.eq(".") {
          hg(vec!["pull"]).await;
        } else {
          hg(vec!["clone", &u, dst.to_str().unwrap()]).await; // this should be fallible
          println!("repo created at {}", dst.display());
        }
      }
      "dm" => println!("sending message to: {}", resource),
      "drive" => {
        let hd = tenex::google::drive_handle(Oauth2Config::default())
          .await
          .unwrap();
        hd.files()
          .list()
          .supports_team_drives(false)
          .supports_all_drives(true)
          //          .corpora("sed")
          .doit()
          .await
          .expect("google_drive failed!");
      }
      "cdn" => {
        let u = format!("https://cdn.rwest.io/{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      }
      "pkg" => {
        let u = format!("https://pkg.rwest.io/{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      }
      "http" => {
        let u = format!("http:{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      }
      "https" => {
        let u = format!("https:{}", &resource);
        download(Url::from_str(&u).unwrap(), &dst).await.unwrap();
      }
      "ssh" => {
        let cfg = SshConfig::default();
        println!("requesting resource over ssh: {}", resource);
      }
      _ => error!("unrecognized server type {:?}", t),
    }
    Ok(())
  }
}

async fn download<P: AsRef<Path>>(url: reqwest::Url, path: P) -> Result<(), NetErr> {
  let res = reqwest::get(url).await?;
  let mut dst = {
    let fname = res
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) })
      .expect("could not create path for url");
    let fname = path.as_ref().join(fname);
    println!("downloading file to {}", fname.display());
    File::create(fname)?
  };
  let content = res.text().await?;
  std::io::copy(&mut content.as_bytes(), &mut dst)?;
  Ok(())
}
