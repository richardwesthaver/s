//! shed/config
//!
//! Shed configurationn layer.

use std::{
  fs,
  io::Write,
  path::{Path, PathBuf},
};

use ::cfg::{
  ron::{
    de::from_reader,
    extensions::Extensions,
    ser::{to_string_pretty, PrettyConfig},
  },
  NetworkConfig, PackageConfig, Result,
};
use hash::{B3Hash, B3Hasher, Id};
use serde::{Deserialize, Serialize};

pub fn load_config(path: &str) {
  let cfg: Config = Config::load(path).unwrap();
  println!("Config: {:?}", cfg);
}

/// Write the given Configuration to a config.ron file.
pub fn write_config(config: Config, output: &Path) {
  config.write(output).expect("should write config to output");
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Config {
  id: String,
  shed_path: PathBuf,
  pkg_path: PathBuf,
  contrib_path: PathBuf,
  pkg_config: Option<Vec<(String, PackageConfig)>>,
  include: Option<PathBuf>,
  network: Option<NetworkConfig>,
}

impl Default for Config {
  // default params are relative
  fn default() -> Self {
    let id = Id::rand();
    let hash = id.state_hash(&mut B3Hasher::new());
    Config {
      id: hash.to_hex(),
      shed_path: PathBuf::from("~/shed"),
      pkg_path: PathBuf::from("pkg"),
      pkg_config: None,
      contrib_path: PathBuf::from("contrib"),
      include: None,
      network: Some(NetworkConfig::default()),
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Config::default()
  }

  pub fn write(&self, path: &Path) -> Result<()> {
    let pretty = PrettyConfig::new()
      .with_indentor("  ".to_owned())
      .with_extensions(Extensions::all());
    let mut file = fs::File::create(path)?;
    let s = to_string_pretty(&self, pretty).expect("Serialization failed");
    write!(file, "{}", s).unwrap();
    println!("wrote to file - {}", path.display());
    Ok(())
  }

  pub fn load(path: &str) -> Result<Self> {
    let f = fs::File::open(path).expect("Failed to read config.ron file.");
    let config: Config = match from_reader(f) {
      Ok(x) => x,
      Err(e) => {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
  pub fn include(path: &str) -> Result<Self> {
    let f = fs::File::open(path).expect("Failed to read config.ron file.");
    let config: Config = match from_reader(f) {
      Ok(x) => x,
      Err(e) => {
        println!("Failed to include config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
}
