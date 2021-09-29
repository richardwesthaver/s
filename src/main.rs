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
use app::App;
mod cli;
use cli::build_cli;
mod config;
use config::Config;
mod repl;
use repl::{dmc, python};
mod ui;
use ui::{stash, store};

// rlib
use rlib::{
  ctx, flate,
  kala::cmd::{midi::list_midi_ports, sys::{describe_host,usb_devices}},
  logger::log::{debug, info, error},
  obj::Objective, obj::config::Configure,
  util::Result};
// tenex
use tenex::{ipapi::get_ip,nws::weather_report};
#[ctx::main]
async fn main() -> Result<()> {
  let cli = build_cli().version(env!("DEMON_VERSION")).get_matches();
  let cfg = match cli.value_of("config") {
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

  if let Some(cmd) = cli.subcommand() {
    match cmd {
      ("pack", opt) => {
        let i = &opt.value_of("input").unwrap();
        let o = &opt.value_of("output").unwrap();
        println!("pack!({} => {}) ", i,o);
        println!("  input: {}", i);
        println!("  output: {}", i);
        flate::pack(i,o,None);
      },
      ("unpack", opt) => {
        let i = &opt.value_of("input").unwrap();
        let o = &opt.value_of("output").unwrap();
        println!("unpack!({} => {}) ", i,o);
        println!("  input: {}", i);
        println!("  output: {}", i);
        if opt.is_present("replace") {
          flate::unpack_replace(i,o);
        } else {
          flate::unpack(i,o);
        }
      },
      ("status", opt) => {
        if opt.is_present("host") {
          describe_host();
        } else if opt.is_present("usb") {
          usb_devices(None)?;
        } else if opt.is_present("ip") {
          get_ip().await?;
        } else if opt.is_present("midi") {
          list_midi_ports()?;
        } else if opt.is_present("weather") {
          weather_report(41.3557, -72.0995).await?;
        }
      },
      ("init", opt) => {
        app.init(opt.value_of("path").unwrap(), opt.value_of("fmt"))?;
      }
      ("serve", opt) => {
        println!("starting server...");
        if let Some(p) = opt.value_of("package") {
          println!("{:#?}", p);
        }
        app.serve(opt.value_of("engine").unwrap()).await?;
      }
      ("pull", opt) => {
        let parent = match opt.value_of("from") {
          Some(i) => i,
          None => ".",
        };
        println!("pulling from {}...", parent);
        app.request("hg", parent).await?;
      }
      ("push", _opt) => {
        unimplemented!();
      }
      ("store", _opt) => {
        println!("running store...");
        store();
      }
      ("stash", _opt) => {
        println!("running stash...");
        stash();
      }
      ("build", _opt) => {
        println!("starting build...");
      }
      ("x", opt) => {
        let m = (opt.value_of("script"), opt.value_of("command"), opt.value_of("module"));
        if let Some(rp) = opt.value_of("repl") {
          match rp {
            "python" | "py" => {
              println!("running python interpreter");
              python::run(|_vm| {}, m.0, m.1, m.2);
            }
            "bqn" => {
              println!("running BQN interpreter");
            }
            "dmc" => {
              println!("running DMC interpreter");
              dmc::run()?;
            }
            _ => {
              println!("unknown REPL type");
            }
          }
        } else {
          println!("running the default interpreter: DMC");
          dmc::run()?;
        }
      },
      (&_, _) => {
        error!("cmd not found");
      }
    }
  } else {
    println!("shed v0.1.0");
  }
  Ok(())
}
