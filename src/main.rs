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
mod repl;
mod ui;

// crate
use app::App;
use cli::{Opts, SubCommand};
use config::Config;
use ui::{stash, store};

// rlib
use rlib::{
  ctx, flate,
  kala::cmd::midi::list_midi_ports,
  kala::cmd::sys::{describe_host, usb_devices},
  logger::log::{debug, info},
  obj::config::Configure,
  obj::Objective,
  util::cli::Clap,
  util::Result,
};

use tenex::ipapi::get_ip;

// std
use std::path::Path;

#[ctx::main]
async fn main() -> Result<()> {
  let opts: Opts = Opts::parse();

  let cfg = match opts.config {
    Some(cfg) => {
      debug!("loading config: {}", cfg);
      Config::load(cfg)?
    }
    None => {
      info!("loading defaults...");
      Config::default()
    }
  };

  let app = App::new(cfg);

  if let Some(i) = opts.subcmd {
    match i {
      SubCommand::Pack(i) => {
        println!("pack!({} => {}) ", &i.input, &i.output);
        println!("  input: {}", &i.input);
        println!("  output: {}", &i.output);
        flate::pack(&i.input, &i.output, None);
      }
      SubCommand::Unpack(i) => {
        println!("running unpack...");
        println!("  input: {}", i.input);
        println!("  output: {}", i.output);
        if i.replace {
          flate::unpack_replace(Path::new(&i.input), Path::new(&i.output));
        } else {
          flate::unpack(Path::new(&i.input), Path::new(&i.output));
        }
      }
      SubCommand::Status(i) => {
        if i.sys == true {
          describe_host();
          println!();
          usb_devices(None)?;
          list_midi_ports()?;
          get_ip().await?;
        }
      }
      SubCommand::Init(i) => {
        app.init(i.path, i.json)?; // i.json is a bool
      }
      SubCommand::Serve(i) => {
        println!("starting server...");
        if let Some(p) = i.packages {
          println!("{:#?}", p);
        }
        app.serve(i.engine).await?;
      }
      SubCommand::Pull(i) => {
        let parent = i.input;
        println!("pulling from {}...", parent);
        app.request("hg".to_string(), parent)?;
      }
      SubCommand::Push(i) => {
        let parent = i.input;
        println!("pushing package {:#?}...", parent);
      }
      SubCommand::Publish(i) => {
        println!("publishing packages {:#?}...", i.packages);
      }
      SubCommand::Store(_) => {
        println!("running store...");
        store();
      }
      SubCommand::Stash(_) => {
        println!("running stash...");
        stash();
      }
      SubCommand::Build(_) => {
        println!("starting build...");
      }
      SubCommand::Meta(_) => {}
      SubCommand::Note(_) => {}
      SubCommand::X(i) => {
        if let Some(rp) = i.repl {
          match rp.as_str() {
            "python" | "py" => {
              println!("running python interpreter");
              repl::python::run(|_vm| {}, i.file, i.cmd);
            }
            "bqn" => {
              println!("running BQN interpreter");
            }
            "dmc" => {
              println!("running DMC interpreter");
              repl::dmc::run()?;
            }
            _ => {
              println!("unknown REPL type");
            }
          }
        } else {
          println!("running the default interpreter: DMC");
          repl::dmc::run()?;
        }
      }
    }
  } else {
    println!("shed v0.1.0");
  }
  Ok(())
}
