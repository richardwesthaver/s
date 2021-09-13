//! config module
//!
//! Shed configuration layer.

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use cfg::{
    config::Configure,
    ron::{
        de::from_reader,
        extensions::Extensions,
        ser::{to_string_pretty, PrettyConfig},
    },
    NetworkConfig, PackageConfig, HgwebConfig, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: PathBuf,
    pub owner: Option<String>,
    pub src: Vec<PackageConfig>,
    pub network: NetworkConfig,
    pub hgrc: HgwebConfig,
}

impl Default for Config {
    // default params are relative
    fn default() -> Self {
        Config {
            path: PathBuf::from(option_env!("SHED_CFG").unwrap_or(".")),
            owner: Some(env!("USER").to_string()),
            src: vec![],
            network: NetworkConfig::default(),
            hgrc: HgwebConfig::default(),
        }
    }
}

impl Config {
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

    pub fn load(path: String) -> Result<Self> {
        let f = fs::File::open(path)?;
        let config: Config = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        Ok(config)
    }
}

impl Configure for Config {}
