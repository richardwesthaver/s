/// bin/shc.rs --- shed-client
use shed::{app::App, cli::build_cli, config::Config};

use rlib::{
  ctx, flate,
  kala::{
    cmd::{
      hg::hg,
      midi::list_midi_ports,
      repl::{apl, bqn, dyalog, erl, k, lua, shakti},
      shell::emacsclient,
      sys::{describe_host, usb_devices},
    },
    dmc, python,
  },
  logger::log::{error, info},
  util::Result,
};

use tenex::{ipapi::get_ip, nws::weather_report};

use std::{env, path::Path};

#[ctx::main]
async fn main() -> Result<()> {
  let cli = build_cli().version(env!("DEMON_VERSION")).get_matches();

  // try
  let cfg = match cli.value_of("config") {
    Some(cfg) => {
      println!("custom cfg: {}", cfg);
      Config::load(cfg)?
    }
    None => {
      let env = Path::new(env!("CFG")).join("shed.cfg");
      if env.is_file() {
        Config::load(env)?
      } else {
        println!("loading defaults...");
        Config::new()
      }
    }
  };

  let log_lvl = cli.occurrences_of("log_level");
  let mut app = App::start(cfg, log_lvl)?;

  if let Some(cmd) = cli.subcommand() {
    match cmd {
      ("pack", opt) => {
        let (i, o) = (
          opt.value_of("input").unwrap(),
          opt.value_of("output").unwrap(),
        );
        if Path::new(i).is_dir() {
          let o = if o.eq(".") {
            format!("{}.{}", i, "tz")
          } else {
            o.to_owned()
          };
          info!("packing: {} => {} ", i, o);
          flate::pack(i, o, None);
        } else if Path::new(i).is_file() {
          let o = if o.eq(".") {
            format!("{}.{}", i, "z")
          } else {
            o.to_owned()
          };
          info!("compressing file: {} => {} ", i, o);
          flate::compress(i, o)?;
        }
      }
      ("unpack", opt) => {
        let (i, o) = (
          &opt.value_of("input").unwrap(),
          &opt.value_of("output").unwrap(),
        );
        if Path::new(i).is_file() {
          println!("unpacking: {} => {} ", i, o);
          if opt.is_present("replace") {
            flate::unpack_replace(i, o);
          } else {
            flate::unpack(i, o);
          }
        }
      }
      ("status", opt) => {
        if opt.is_present("sys") {
          describe_host();
        }
        if opt.is_present("usb") {
          usb_devices(None)?;
        }
        if opt.is_present("ip") {
          get_ip().await?;
        }
        if opt.is_present("midi") {
          list_midi_ports()?;
        }
        if opt.is_present("weather") {
          weather_report(41.3557, -72.0995).await?;
        }
        if opt.is_present("vc") {
          match opt.value_of("input") {
            Some(i) => {
              let cd = env::current_dir()?;
              env::set_current_dir(&Path::new(i))?;
              if opt.is_present("remote") {
                hg(&["summary", "--remote"]).await?;
                hg(&["status"]).await?;
              } else {
                hg(&["summary"]).await?;
                hg(&["status"]).await?;
              }
              env::set_current_dir(cd)?;
            }
            None => {
              if opt.is_present("remote") {
                hg(&["summary", "--remote"]).await?;
                hg(&["status"]).await?;
              } else {
                hg(&["summary"]).await?;
                hg(&["status"]).await?;
              }
            }
          }
        }
      }
      ("init", opt) => {
        if opt.is_present("db") {
          app.init_db()?;
        } else {
          app.init_cfg(opt.value_of("fmt"))?;
        }
      }
      ("serve", opt) => {
        println!("starting server...");
        if let Some(p) = opt.value_of("package") {
          println!("{:#?}", p);
        }
        app.serve(opt.value_of("engine").unwrap()).await?;
      }
      ("download", opt) => {
        match opt.value_of("input") {
          Some(i) => {
            let s: Vec<&str> = i.split(":").collect();
            info!("downloading {} from {}...", s[1], s[0]);
            app.dl(s[0], s[1]).await?;
          }
          None => {
            error!("an object URI is required!")
          }
        };
      }
      ("push", _opt) => {
        hg(&["push"]).await?;
      }
      ("pull", _opt) => {
        hg(&["pull"]).await?;
      }
      ("store", _opt) => {
        println!("running store...");
      }
      ("stash", _opt) => {
        println!("running stash...");
      }
      ("build", opt) => {
        println!("starting build...");
        match opt.value_of("pkg") {
          Some(i) => {
            app
              .build_src(opt.value_of("target").unwrap_or("o"), i)
              .await?
          }
          None => {
            app
              .build_src(opt.value_of("target").unwrap_or("o"), ".")
              .await?
          }
        }
      }
      ("x", opt) => {
        let it = opt.value_of("interpreter");
        let mut args: Vec<&str> = vec![];
        let _input = opt.value_of("input");
        if let Some(rp) = opt.value_of("repl") {
          match rp {
            "python" | "py" => {
              println!("running python interpreter");
              python::run(|_vm| {}, opt);
            }
            "bqn" => {
              println!("running BQN interpreter");
              if let Some(f) = opt.values_of("script") {
                args.insert(0, "-f");
                for (x, i) in f.enumerate() {
                  args.insert(x, i);
                }
                bqn(args).await?;
              } else if let Some(f) = opt.values_of("command") {
                args.insert(0, "-p");
                for (x, i) in f.enumerate() {
                  args.insert(x, i);
                }
                bqn(args).await?;
              } else {
                args.insert(0, "-r");
                bqn(args).await?;
              }
            }
            "elisp" | "el" => {
              println!("running IELM");
              emacsclient(vec!["-t", "-e", "(ielm)"]).await?;
            }
            "k" => {
              if let Some("k9") = opt.value_of("interpreter") {
                println!("running shakti (k9) interpreter");
                shakti(vec![]).await?;
              } else {
                println!("running ngn/k (k6) interpreter");
                k(vec![]).await?;
              }
            }
            "erl" => {
              println!("running Erlang interpreter");
              erl(vec![]).await?;
            }
            "apl" => {
              if let Some("gnu") = it {
                apl(vec![]).await?;
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
              if let Some(i) = opt.value_of("command") {
                args.append(vec!["-e", i].as_mut());
              }
              if let Some(i) = opt.value_of("module") {
                args.append(vec!["-l", i].as_mut());
              }
              if let Some(i) = opt.value_of("script") {
                args.append(vec![i].as_mut());
              }
              info!("running Lua interpreter");
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
      ("edit", opt) => match opt.value_of("input") {
        Some(i) => {
          app.edit(i).await?;
        }
        None => {
          app.edit(".").await?;
        }
      },
      ("clean", _opt) => {}
      (&_, _) => {
        error!("cmd not found");
      }
    }
  } else {
    error!("no command supplied");
  }
  Ok(())
}
