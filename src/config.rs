//! config module
//!
//! Shed configuration layer.

use std::{
  fs,
  path::{Path, PathBuf},
  collections::HashMap,
};

use crate::{Configure, Objective};
use rlib::{logger::log::error, obj::{
  Result,
  config::{
    network::NetworkConfig,
    package::PackageConfig,
    repo::hg::{MercurialConfig, HgwebConfig},
  },
  ron::de::from_reader,
}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: PathBuf, // the shed path on disk
    pub owner: Option<String>,
    pub src: Vec<PackageConfig>,
    pub network: NetworkConfig,
    pub hgrc: MercurialConfig,
}

impl Default for Config {
  fn default() -> Self {
    let mut ui = HashMap::new();
    ui.insert("username".to_string(), "ellis <ellis@rwest.io>".to_string());
    let hgrc = MercurialConfig {
      ui,
      extensions: None,
      paths: None,
      web: HgwebConfig::default(),
    };
    Config {
      path: PathBuf::from(option_env!("SHED").unwrap_or(std::env::current_dir().unwrap().to_str().unwrap())),
      owner: Some(env!("USER").to_string()),
      src: vec![],
      network: NetworkConfig::default(),
      hgrc,
    }
  }
}

impl Config {
  pub fn write<P: AsRef<Path>>(&self, path: P, ext: Option<&str>) -> Result<()> {
    let path = path.as_ref();
    let f_path = &path.join(".shed");
    let file = fs::File::create(f_path)?;
    match ext {
      Some(i) => {
        match i {
          "json" => {
            self.to_json_writer(file)?
          },
          "ron" => {
            self.to_ron_writer(file)?
          },
          i => {
            error!("extension '{}' not understood", i);
            std::process::exit(1);
          },
        }
      },
      None => {
        self.to_ron_writer(file)?
      },
    }
    println!("wrote config to {}", f_path.display());
    Ok(())
  }

  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    let f = fs::File::open(path)?;
    let config: Config = match from_reader(f) {
      Ok(x) => x,
      Err(e) => {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
}

impl Configure for Config {}
impl Objective for Config {}
