//! # shed
//!
//! A Shed is a nested structure used to store collections of
//! development resources such as code, configs, docs, and data.
//!
//! This program is used to bootstrap, configure, and manage a local
//! Shed.
mod app;
mod config;

use util::cli::{AppSettings, ArgSettings, Clap};
use util::Result;

#[derive(Debug)]
pub enum Cmd {
    /// pack a Package from the Registry in tar.zst format
    Pack,
    /// unpack a tar.zst compressed archive
    Unpack,
    /// report the current shed status
    Status,
    /// pull changesets from a remote
    Pull,
    /// update local copy and refresh cache
    Update,
    /// host shed network services
    Serve,
}

#[ctx::main]
async fn main() -> Result<()> {
    logger::flexi("info")?;

    Ok(())
}
