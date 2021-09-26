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
mod ui;
//mod repl;

// crate
use app::App;
use cli::{Opts, SubCommand};
use config::Config;
//use repl::python::run;
use ui::{stash, store};

// rlib
use rlib::{
  ctx, flate,
  util::Result,
  util::cli::Clap,
  obj::config::Configure,
  obj::Objective,
  kala::cmd::sys::{describe_host, usb_devices},
  kala::cmd::midi::list_midi_ports,
  kala::cmd::input::rustyline,
  logger::log::{debug, info},
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
    },
    None => {
      info!("loading defaults...");
      Config::default()
    },
  };

  let app = App::new(cfg);

  if let Some(i) = opts.subcmd {
    match i {
      SubCommand::Pack(i) => {
        println!("running pack...");
        println!("  input: {}", i.input);
        println!("  output: {}", i.output);
        flate::pack(&i.input, &i.output, None);
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
      SubCommand::Status(i) => {
        if i.sys == true {
          describe_host();
          usb_devices(None)?;
          list_midi_ports()?;
          get_ip().await?;
        }
      },
      SubCommand::Init(i) => {
        app.init(i.path, i.json)?;
      }
      SubCommand::Serve(i) => {
        println!("starting server...");
        if let Some(p) = i.packages {
          println!("{:#?}", p);
        }
        app.serve(i.engine).await?;
      },
      SubCommand::Pull(i) => {
        let parent = i.input;
        println!("pulling from {}...", parent);
        app.request("hg".to_string(), parent)?;
      },
      SubCommand::Push(i) => {
        let parent = i.input;
        println!("pushing package {:#?}...", parent);
      },
      SubCommand::Publish(i) => {
        println!("publishing packages {:#?}...", i.packages);
      },
      SubCommand::Store(_) => {
        println!("running store...");
        store();
      },
      SubCommand::Stash(_) => {
        println!("running stash...");
        stash();
      },
      SubCommand::Build(_) => {
        println!("starting build...");
      },
      SubCommand::Meta(_) => {
      },
      SubCommand::Note(_) => {
      },

    }
  } else {
//    python::run(|_| {});
    let mut rl = rustyline::Editor::<()>::new();
    let input = rl.readline("> ")?;
    println!("Input: {}", input);
  }
  Ok(())
}
