//! client.rs --- shed client application launcher
/**
`ArgMatches` parsed from 'cli::build_cli()' are passed to the 'Client'
with a 'cfg::Config'. This type is used to call other functions from
'rlib', 'tenex', and internal modules.
*/
use crate::Config;

use rlib::{
  db::{registry::Registry, Error as DbErr},
  flate,
  kala::{
    cmd::{
      hg::{hg, hgweb},
      midi::list_midi_ports,
      shell::{emacsclient, make},
      sys::{describe_host, usb_devices},
    },
    Error as KErr,
  },
  logger::log::{error, info},
  net::{
    reqwest::{Client, Url},
    Error as NetErr,
  },
  obj::Error,
  util::{cli::ArgMatches, Result},
};

use tenex::{ipapi::get_ip, nws::weather_report};

use std::{
  env,
  fs::{create_dir, remove_file, File},
  path::{Path, PathBuf},
  str::FromStr,
};

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

/// shc application
pub struct App<'a> {
  /// User configuration
  pub cfg: Config,
  /// CLI args
  pub cli: &'a ArgMatches,
}

impl<'a> App<'a> {
  /// Generate a new `App` instance from CLI args
  pub fn new(cli: &'a ArgMatches) -> Result<Self, KErr> {
    // set config
    let cfg = match cli.value_of("config") {
      Some(cfg) => {
        info!("custom cfg: {}", cfg);
        Config::load(cfg)?
      }
      None => {
        let env = Path::new(env!("CFG")).join("shed.cfg");
        if env.is_file() {
          Config::load(env)?
        } else {
          Config::new()
        }
      }
    };

    info!("App Config: {:?}", cfg);

    let log_lvl = cli.occurrences_of("log_level");
    let lvl = match log_lvl {
      0 => "warn",
      1 => "info",
      2 => "debug",
      3.. => "trace",
    };

    let shed_path: PathBuf = cfg.path.to_path_buf();
    match shed_path.join("data/log").to_str() {
      Some(p) => {
        rlib::logger::file(lvl, p, "shc").expect("logger init failed");
      }
      None => rlib::logger::flexi(lvl).expect("logger init failed"),
    };

    Ok(App { cfg, cli })
  }

  /// Matches on any subcommands and execute additional methods
  /// accordingly.
  pub async fn dispatch(&'a self) -> Result<()> {
    if let Some(cmd) = self.cli.subcommand() {
      match cmd {
        ("init", opt) => {
          if opt.is_present("db") {
            self.init_db()?;
          } else {
            self.init_cfg()?;
          }
        }
        ("build", _) => {
          println!("starting build...");
          self.build_src().await?
        }
        // Status
        ("status", opt) => {
          if opt.is_present("sys") {
            describe_host();
          }
          if opt.is_present("usb") {
            usb_devices(None)?;
          }
          if opt.is_present("ip") {
            get_ip().await?;
          }
          if opt.is_present("midi") {
            list_midi_ports()?;
          }
          if opt.is_present("weather") {
            weather_report(41.3557, -72.0995).await?;
          }
          if opt.is_present("vc") {
            match opt.value_of("input") {
              Some(i) => {
                let cd = env::current_dir()?;
                env::set_current_dir(&Path::new(i))?;
                if opt.is_present("remote") {
                  hg(&["summary", "--remote"]).await?;
                  hg(&["status"]).await?;
                } else {
                  hg(&["summary"]).await?;
                  hg(&["status"]).await?;
                }
                env::set_current_dir(cd)?;
              }
              None => {
                if opt.is_present("remote") {
                  hg(&["summary", "--remote"]).await?;
                  hg(&["status"]).await?;
                } else {
                  hg(&["summary"]).await?;
                  hg(&["status"]).await?;
                }
              }
            }
          }
        }
        // Version Control
        ("push", _opt) => {
          hg(&["push"]).await?;
        }
        ("pull", _opt) => {
          hg(&["pull"]).await?;
        }
        // Networking
        ("download", opt) => {
          match opt.value_of("input") {
            Some(i) => {
	      // NOTE: splitting on ':' generally isn't a good idea,
	      // it splits common values like 'https://site.com' into
	      // ['https', '//site.com']. Don't forget to account for
	      // this in the parsing function!
              let s: Vec<&str> = i.split(":").collect(); 
              info!("downloading {} from {}...", s[1], s[0]);
              self.dl(s[0], s[1]).await?;
            }
            None => {
              error!("an object URI is required!");
            }
          };
        }
        ("serve", opt) => {
          println!("starting server...");
          if let Some(p) = opt.value_of("package") {
            println!("{:#?}", p);
          }
          self.serve().await?;
        }
        // Compression
        ("pack", opt) => {
          let (i, o) = (
            opt.value_of("input").unwrap(),
            opt.value_of("output").unwrap(),
          );
          if Path::new(i).is_dir() {
            let o = if o.eq(".") {
              format!("{}.{}", i, "tz")
            } else {
              o.to_owned()
            };
            info!("packing: {} => {} ", i, o);
            flate::pack(i, o, None);
          } else if Path::new(i).is_file() {
            let o = if o.eq(".") {
              format!("{}.{}", i, "z")
            } else {
              o.to_owned()
            };
            info!("compressing file: {} => {} ", i, o);
            flate::compress(i, o)?;
          }
        }
        ("unpack", opt) => {
          let (i, o) = (
            &opt.value_of("input").unwrap(),
            &opt.value_of("output").unwrap(),
          );
          if Path::new(i).is_file() {
            println!("unpacking: {} => {} ", i, o);
            if opt.is_present("replace") {
              flate::unpack_replace(i, o);
            } else {
              flate::unpack(i, o);
            }
          }
        }
        ("edit", _) => self.edit().await?,
        ("clean", _opt) => {}
        (&_, _) => {
          error!("cmd not found");
        }
      }
    }
    Ok(())
  }

  /// Create the `shed` directory tree
  pub fn build_dirs(self) -> Result<()> {
    let base = &self.cfg.path;
    create_dir(base)?;
    for i in ["stash", "store", "src", "lab", "data", "data/log"] {
      create_dir(base.join(i))?;
    }
    Ok(())
  }

  /// Build source code
  pub async fn build_src(&'a self) -> Result<(), KErr> {
    make(vec![self.cli.value_of("target").unwrap()]).await?;
    Ok(())
  }

  /// Initialize a configuration from cli
  pub fn init_cfg(&'a self) -> Result<()> {
    let p: PathBuf = match self.cli.value_of("path") {
      Some(p) => p.into(),
      None => "~/.config/shed.cfg".into(),
    };
    if !p.exists() {
      println!("writing shed.cfg to {}...", p.display());
      match self.cli.value_of("fmt") {
        Some("ron") | None => self.cfg.write(&p, None)?,
        Some("json") => self.cfg.write(&p, Some("json"))?,
        Some("bin") => self.cfg.write(&p, Some("bin"))?,
        Some(_) => error!("unknown configuration type"),
      }
    } else if self.cli.is_present("force") {
      //  TODO 2021-11-04: overwrite existing config : requires --auto prompt
    } else {
      error!("{} already exists, use -f to override", p.display());
    }
    Ok(())
  }

  /// Initialize the database
  pub fn init_db(&self) -> Result<(), DbErr> {
    let db_path: PathBuf = self.cfg.path.clone().join("data/db");
    std::fs::remove_dir_all(&db_path)?;
    Registry::new(&db_path)?;
    Ok(())
  }

  /// Open $EDITOR
  pub async fn edit(&'a self) -> Result<(), Error> {
    let input = self.cli.value_of("input").unwrap();
    if input.eq("cfg") {
      let cfg = option_env!("SHED_CFG").unwrap();
      emacsclient(vec!["-t", cfg]).await.unwrap();
    } else {
      emacsclient(vec!["-t", input]).await.unwrap();
    }
    Ok(())
  }

  /// Clean up shed resources
  pub async fn clean(&'a self) -> Result<()> {
    match self.cli.value_of("input") {
      Some("cfg") => remove_file("~/.config/shed.cfg")?,
      Some("log") => remove_file(self.cfg.path.join("data/log/shed.log"))?,
      _ => {
        for i in self.cfg.src.iter() {
          println!("not actually removing {}, silly", i.name);
        }
      }
    }
    Ok(())
  }

  /// Start an external server
  pub async fn serve(&'a self) -> Result<()> {
    match self.cli.value_of("engine") {
      Some("hg") => {
        hgweb(&self.cfg.hg).await?;
        Ok(())
      }
      Some("dm") => Ok(println!("waiting for dm...")),
      Some(_) | None => Ok(error!("unrecognized server type!")),
    }
  }
  /// Download a remote resource
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
              println!("found auth: {:?}", i);
              todo!("handle oauth and send to tenex");
            }
          }
        }
        todo!("see drive handler above");
      }
      "a" => {
        let u = format!("https://rwest.io/a/{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "y" => {
        let u = format!("https://rwest.io/y/{}", &resource);
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "http" => {
          let u = if &resource[..2] == "//" {
	    format!("http:{}", &resource)
	  } else {
	    format!("http://{}", &resource)
	  };
	
        download(&client, Url::from_str(&u).unwrap(), &dst)
          .await
          .unwrap();
      }
      "https" => {
        let u = format!("https:{}", &resource);
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
