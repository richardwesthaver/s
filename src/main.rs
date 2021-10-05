//! # shed
//!
//! A Shed is a collections of development resources such as code,
//! configs, docs, and data. It is intended to be an `org-specific`
//! structure which is maintained internally and encapsulates all of
//! the best native development practices - good or bad.
//!
//! This program is a development tool I use to manage such
//! structures.
#![feature(drain_filter)]

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

//rlib
use rlib::{
  ctx, flate,
  kala::cmd::{
    hg::hg,
    midi::list_midi_ports,
    repl::{bqn, dyalog, erl, gnu_apl, k, k9, lua},
    sys::{describe_host, usb_devices},
  },
  logger::log::{debug, error, info},
  obj::config::Configure,
  obj::Objective,
  util::Result,
};
//tenex
use tenex::{ipapi::get_ip, nws::weather_report};

//std
use std::{env, path::Path};

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

  let mut app = App::new(cfg);

  if let Some(cmd) = cli.subcommand() {
    match cmd {
      ("pack", opt) => {
        let i = &opt.value_of("input").unwrap();
        let o = &opt.value_of("output").unwrap();
        println!("packing dir: {} => {} ", i, o);
        flate::pack(i, o, None);
      }
      ("unpack", opt) => {
        let i = &opt.value_of("input").unwrap();
        let o = &opt.value_of("output").unwrap();
        println!("unpacking pkg: {} => {} ", i, o);
        if opt.is_present("replace") {
          flate::unpack_replace(i, o);
        } else {
          flate::unpack(i, o);
        }
      }
      ("status", opt) => {
        if opt.is_present("sys") {
          describe_host();
        } else if opt.is_present("usb") {
          usb_devices(None)?;
        } else if opt.is_present("ip") {
          get_ip().await?;
        } else if opt.is_present("midi") {
          list_midi_ports()?;
        } else if opt.is_present("weather") {
          weather_report(41.3557, -72.0995).await?;
        } else {
          let input = opt.value_of("input").unwrap_or(".");
          let cd = env::current_dir()?;
          env::set_current_dir(&Path::new(input))?;
          hg(vec!["summary"]).await;
          if opt.is_present("remote") {
            println!("#@! status :: \n");
            hg(vec!["status", "--remote"]).await; //needs error handling
          } else {
            hg(vec!["status"]).await;
          }
          env::set_current_dir(cd)?;
        }
      }
      ("init", opt) => {
        if opt.is_present("db") {
          app.db_init()?;
        } else {
          app.init(opt.value_of("path").unwrap(), opt.value_of("fmt"))?;
        }
      }
      ("serve", opt) => {
        println!("starting server...");
        if let Some(p) = opt.value_of("package") {
          println!("{:#?}", p);
        }
        app.serve(opt.value_of("engine").unwrap()).await?;
      }
      ("pull", opt) => {
        let (ty, r) = match opt.value_of("input") {
          Some(i) => {
            let mut s = i.split(":");
            (s.next().unwrap(), s.next().unwrap())
          }
          None => ("hg", "."),
        };
        println!("pulling {} from {}...", r, ty);
        app.request(ty, r).await?;
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
      ("build", opt) => {
        println!("starting build...");
        match opt.value_of("pkg") {
          Some(i) => app.build(opt.value_of("target").unwrap_or("o"), i).await?,
          None => {
            app
              .build(opt.value_of("target").unwrap_or("o"), ".")
              .await?
          }
        }
      }
      ("x", opt) => {
        let m = (
          opt.value_of("script"),
          opt.value_of("command"),
          opt.value_of("module"),
        );
        let it = opt.value_of("interpreter");
        let mut args: Vec<&str> = vec![];
        let _input = opt.value_of("input");
        if let Some(rp) = opt.value_of("repl") {
          match rp {
            "python" | "py" => {
              println!("running python interpreter");
              python::run(|_vm| {}, m.0, m.1, m.2);
            }
            "bqn" => {
              println!("running BQN interpreter");
              if let Some(f) = m.0 {
                args.insert(0, "-f");
                args.insert(1, f);
                bqn(args).await?;
              } else if let Some(x) = m.1 {
                args.insert(0, "-p");
                args.insert(1, x);
                bqn(args).await?;
              } else {
                args.insert(0, "-r");
                bqn(args).await?;
              }
            }
            "k" => {
              if let Some("k9") = it {
                println!("running shakti (k9) interpreter");
                if m.0.is_some() {
                  k9(args).await?;
                } else {
                  println!("running ngn/k (k6) interpreter");
                  k(args).await?;
                }
              }
            }
            "erl" => {
              println!("running Erlang interpreter");
              erl(vec![]).await?;
            }
            "apl" => {
              if let Some("gnu") = it {
                gnu_apl(vec![]).await?;
              } else {
                println!("running APL interpreter: Dyalog");
                dyalog(vec!["-b"]).await?;
              }
            }
            "dmc" => {
              println!("running DMC interpreter");
              dmc::run()?;
            }
            "lua" => {
              println!("running Lua interpreter");
              args.insert(0, "-i");
              if m.0.is_some() {
                args.append(vec!["--", m.0.unwrap()].as_mut());
              }
              if m.1.is_some() {
                args.append(vec!["-e", m.1.unwrap()].as_mut())
              }
              if m.2.is_some() {
                args.append(vec!["-l", m.2.unwrap()].as_mut());
              }
              lua(args).await?;
            }
            _ => {
              println!("unknown REPL type");
            }
          }
        } else {
          println!("running the default interpreter: DMC");
          dmc::run()?;
        }
      }
      (&_, _) => {
        error!("cmd not found");
      }
    }
  } else {
    debug!("no command supplied");
  }
  Ok(())
}
