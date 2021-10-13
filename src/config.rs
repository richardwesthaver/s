//! config module
//!
//! Shed configuration layer.

use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
};

use crate::{Configure, Objective};

use rlib::{
  logger::log::{error, info},
  kala::cmd::shell::emacsclient,
  obj::{
    config::{HgwebConfig, MercurialConfig, NetworkConfig, PackageConfig, Oauth2Config, SshConfig, UserConfig, ProjectConfig, ProgramConfig},
    ron::de::from_reader,
    Result,
    impl_config,
  },
};

use serde::{Deserialize, Serialize};

/// Shed configuration type
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub path: PathBuf, // the shed path on disk
  pub src: Vec<PackageConfig>,
  pub bin: Vec<ProgramConfig>,
  pub net: NetworkConfig,
  pub hg: MercurialConfig,
  pub lab: Vec<ProjectConfig>,
  pub usr: UserConfig,
}

impl Config {
  pub fn new() -> Self {
    let hg = MercurialConfig {
      ui: HashMap::new(),
      extensions: None,
      paths: None,
      web: HgwebConfig::default(),
    };
    let lab = vec![];
    let usr = UserConfig::default();
    Config {
      path: PathBuf::from(
        option_env!("SHED").unwrap_or("~/shed")
      ),
      src: vec![],
      bin: vec![],
      net: NetworkConfig::default(),
      hg,
      lab,
      usr
    }
  }

  pub fn write<P: AsRef<Path>>(&self, path: P, ext: Option<&str>) -> Result<()> {
    let path = path.as_ref();
    let f_path = &path.join("shed.cfg");
    let file = fs::File::create(f_path)?;
    match ext {
      Some(i) => match i {
        "json" => self.to_json_writer(file)?,
        "ron" => self.to_ron_writer(file)?,
        "bin" => self.encode_into(file)?,
        i => {
          error!("extension '{}' not understood", i);
          std::process::exit(1);
        }
      },
      None => self.to_ron_writer(file)?,
    }
    println!("wrote config to {}", f_path.display());
    Ok(())
  }

  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    let f = fs::File::open(path)?;
    let config: Config = match from_reader(f) {
      Ok(x) => {
        info!("loading config: {:?}", x);
        x
      },
      Err(e) => {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
}

impl_config!(Config);

