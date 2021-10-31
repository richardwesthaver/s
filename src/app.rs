//! app.rs --- shed application launcher
/**
Args parsed from 'cli::build_cli()' are passed to the 'App' with a
'cfg::Config', which is handled immediately or passed on to an AppHandle.
or handles them directly.
*/
use crate::config::Config;

use rlib::{
  db::{registry::Registry, Error as DbErr},
  kala::{
    cmd::{
      hg::{hg, hgweb},
      shell::{emacsclient, make},
    },
    Error as KErr,
  },
  logger::log::{error, info},
  net::{
    reqwest::{Client, Url},
    Error as NetErr,
  },
  obj::Error,
  util::Result,
};

use tenex::client::google::Scope;

use std::{
  fs::{create_dir, remove_file, File},
  path::{Path, PathBuf},
  str::FromStr,
};

pub struct App {
  pub cfg: Config,
  //  pub  args:
}

impl App {
  pub fn start(cfg: Config, log_lvl: u64) -> Result<Self, KErr> {
    info!("App Config: {:?}", cfg);
    let shed_path: PathBuf = cfg.path.clone();
    let lvl = match log_lvl {
      0 => "warn",
      1 => "info",
      2 => "debug",
      3.. => "trace",
    };
    match shed_path.join("data/log").to_str() {
      Some(p) => {
        rlib::logger::file(lvl, p, "shc").expect("logger init failed");
      }
      None => rlib::logger::flexi(lvl).expect("logger init failed"),
    };
    println!("{:#?}", cfg);
    Ok(App { cfg })
  }

  pub fn build_dirs(self) -> Result<()> {
    let base = &self.cfg.path;
    create_dir(base)?;
    for i in ["stash", "store", "src", "lab", "data", "data/log"] {
      create_dir(base.join(i))?;
    }
    Ok(())
  }

  pub fn init_cfg(&self, fmt: Option<&str>) -> Result<()> {
    let p = Path::new(env!("CFG")).join("shed.cfg");
    if !p.exists() {
      println!("writing shed.cfg to {}...", p.display());
      match fmt {
        Some("ron") | None => self.cfg.write(&p, None)?,
        Some("json") => self.cfg.write(&p, Some("json"))?,
        Some("bin") => self.cfg.write(&p, Some("bin"))?,
        Some(_) => error!("unknown configuration type"),
      }
    } else {
      error!("{} already exists", p.display());
    }
    Ok(())
  }

  pub async fn build_src(&mut self, target: &str, pkg: &str) -> Result<(), KErr> {
    if self.cfg.src.drain_filter(|src| src.name != *pkg).count() > 0 {
      println!("matched packages");
    };
    make(vec![target]).await?;
    Ok(())
  }

  /// re-initialize the database
  pub fn init_db(&self) -> Result<(), DbErr> {
    let db_path: PathBuf = self.cfg.path.clone().join("data/db");
    std::fs::remove_dir_all(&db_path)?;
    Registry::new(&db_path)?;
    Ok(())
  }

  pub async fn edit(self, input: &str) -> Result<(), Error> {
    if input.eq("cfg") {
      let cfg = option_env!("SHED_CFG").unwrap();
      emacsclient(vec!["-t", cfg]).await.unwrap();
    } else {
      emacsclient(vec!["-t", input]).await.unwrap();
    }
    Ok(())
  }

  pub async fn clean(self, input: &str) -> Result<()> {
    match input {
      "cfg" => remove_file(option_env!("SHED_CFG").expect("SHED_CFG should be set."))?,
      "log" => remove_file(self.cfg.path.join("data/log/shed.log"))?,
      _ => {
        for i in self.cfg.src.iter() {
          println!("not actually removing {}, silly", i.name);
        }
      }
    }
    Ok(())
  }
  pub async fn serve(&self, engine: &str) -> Result<()> {
    match engine {
      "hg" => {
        hgweb(&self.cfg.hg).await?;
        Ok(())
      }
      "dm" => Ok(println!("waiting for dm...")),
      _ => Ok(error!("unrecognized server type!")),
    }
  }

  pub async fn dl(&self, t: &str, resource: &str) -> Result<(), NetErr> {
    let dst = self.cfg.path.join("stash/tmp/");
    let client = Client::new();
    match t {
      "hg" => {
        let u = format!("https://hg.rwest.io/{}", &resource);
        if resource.eq(".") {
          hg(&["pull"]).await?;
        } else {
          hg(&["clone", &u, dst.to_str().unwrap()]).await?;
          println!("repo created at {}", dst.display());
        }
      }
      "dm" => println!("sending message to: {}", resource),
      "drive" => {
        let auth = &self.cfg.usr.auth;
        if auth.is_empty() {
          error!("no AuthConfig!");
        } else {
          for i in auth.into_iter() {
            if i.provider.starts_with("google") || i.oauth.is_some() {
              let hub = tenex::google::drive_handle(
                i.oauth
                  .to_owned()
                  .expect("failed to parse google oauth config")
                  .into(),
              )
              .await
              .unwrap();
              let (r, q) = hub
                .files()
                .list()
                .supports_all_drives(true)
                .q(format!("name = '{}'", resource).as_str())
                .doit()
                .await
                .expect("google_drive failed!");
              info!("file_list status: {}", r.status());
              let f = hub
                .files()
                .get(
                  q.files
                    .unwrap()
                    .first()
                    .unwrap()
                    .id
                    .as_ref()
                    .unwrap()
                    .as_str(),
                )
                .param("alt", "media")
                .add_scope(Scope::Full)
                .doit()
                .await
                .unwrap();
              println!("{:?}", f.0.body());
            }
          }
        }
      }
      "cdn" => {
        let u = format!("https://rwest.io/a/{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "pkg" => {
        let u = format!("https://rwest.io/y/{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "http" => {
        let u = format!("http://{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "https" => {
        let u = format!("https://{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "ssh" => {
        println!("requesting resource over ssh: {}", resource);
      }
      _ => error!("unrecognized server type {:?}", t),
    }
    Ok(())
  }
}

/// HTTP file download client
async fn download<P: AsRef<Path>>(client: &Client, url: Url, path: P) -> Result<(), NetErr> {
  let res = client.get(url).send().await?;
  let mut dst = {
    let fname = res
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) });
    let fname = path
      .as_ref()
      .join(fname.expect("failed to parse path from url"));
    File::create(fname)?
  };
  let content = res.text().await?;
  std::io::copy(&mut content.as_bytes(), &mut dst)?;
  Ok(())
}

pub trait AppHandle {}
