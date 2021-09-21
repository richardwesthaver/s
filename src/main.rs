//! # shed
//!
//! A Shed is a collections of development resources such as code,
//! configs, docs, and data. It is intended to be an `org-specific`
//! structure which is maintained internally and encapsulates all of
//! the best native development practices - good or bad.
//!
//! This program is a development tool I use to manage such
//! structures.
mod app;
mod cli;
mod config;
// crate
use app::App;
use cli::{Opts, SubCommand};
use config::Config;
// rlib
use util::{Result};
use logger::log::info;
use obj::config::Configure;
// contrib
use clap::Clap;
// std
use std::path::Path;

#[ctx::main]
async fn main() -> Result<()> {
  logger::flexi("info")?;
  let opts: Opts = Opts::parse();

  let app = match Config::load(opts.config) {
    Ok(i) => {
      info!("{}", i.to_ron_string()?);
      App::new(i)
    },
    Err(e) => {
      eprintln!("{}", e);
      App::new(Config::default())
    },
  };

  if let Some(i) = opts.subcmd {
    match i {
      SubCommand::Pack(i) => {
        println!("running pack...");
        println!("  input: {}", i.input);
        println!("  output: {}", i.output);
        flate::pack(Path::new(&i.input), Path::new(&i.output), None);
      },
      SubCommand::Unpack(i) => {
        println!("running unpack...");
        println!("  input: {}", i.input);
        println!("  output: {}", i.output);
        if i.replace {
        flate::unpack_replace(Path::new(&i.input), Path::new(&i.output));
        } else {
          flate::unpack(Path::new(&i.input), Path::new(&i.output));
        }
      },
      SubCommand::Status(_) => {
        println!("running status...");
      },
      SubCommand::Init(i) => {
        println!("running init...");
        app.cfg.write(&Path::new(&i.input).join("cfg.ron"))?;
      },
      SubCommand::Serve(i) => {
        println!("serving {}...", i.ty);
        if let Some(p) = i.packages {
          println!("{:#?}", p);
        }
        app.serve(i.ty)?;
      },
      SubCommand::Pull(i) => {
        println!("running pull...");
        app.request("hg:pull".to_string(), i.parent.unwrap())?;
      },
      SubCommand::Publish(i) => {
        println!("publishing packages {:#?}...", i.packages);
      },
      SubCommand::Store(_) => {
        println!("running store...");
      },
      SubCommand::Stash(_) => {
        println!("running stash...");
      },
    }
  }

  Ok(())
}
