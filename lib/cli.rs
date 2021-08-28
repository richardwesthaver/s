use std::path::Path;

use cfg::{NetworkConfig, PackageConfig, ShedConfig};

use super::{
  cmd::{load_cfg_cmd, make_cmd, pack_cmd, write_cfg_cmd},
  App, AppSettings, Arg, Cmd, SubCommand,
};
use crate::ctl::{load_config, pack, shell::make, write_config};
use crate::Result;

/// CLI container for Shed
pub struct ShedCli {
  args: Vec<Arg>,
  cmds: Vec<Cmd>,
}

impl ShedCli {
  pub fn new() -> Self {
    let args = ShedArgs::new();
    let cmds = vec![write_cfg_cmd(), load_cfg_cmd(), make_cmd(), pack_cmd()];

    ShedCli { args: args.0, cmds }
  }

  pub fn build(&self) -> Result<Cmd> {
    let cmds = &self.cmds;
    let args = &self.args;
    let app = App::new("shedctrl")
      .author("ellis")
      .about("Tools for gardening and other associated but occassically abstract polymorphisms.")
      .global_settings(&[
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
        AppSettings::UnifiedHelpMessage,
        AppSettings::AllowMissingPositional,
      ])
      .args(args)
      .subcommands(cmds.clone());

    Ok(app)
  }

  /// Start the CLI and dish out commands to the proper handlers
  pub fn run(&self) -> Result<()> {
    let app = self.build()?;
    let matches = app.get_matches();

    // pack command
    if let Some(matches) = matches.subcommand_matches("pack") {
      if matches.is_present("input") {
        log::info!(
          "input: {}",
          matches.value_of("input").expect("illegal input!")
        );
        pack(
          std::path::Path::new(matches.value_of("input").unwrap()),
          std::path::Path::new(matches.value_of("output").unwrap()),
        );
      } else {
        let cd = std::env::current_dir().unwrap();
        log::info!("using default input");
        pack(
          cd.as_path(),
          std::path::Path::new(matches.value_of("output").unwrap()),
        );
      }
    };

    // write command
    if let Some(matches) = matches.subcommand_matches("write") {
      if matches.is_present("output") {
        write_config(
          ShedConfig::load("config.ron").unwrap(),
          Path::new(matches.value_of("output").unwrap()),
        );
      } else {
        write_config(
          ShedConfig::load("config.ron").unwrap(),
          Path::new("config.ron"),
        );
      }
    };

    // make command
    if let Some(matches) = matches.subcommand_matches("make") {
      if matches.is_present("target") {
        make(matches.value_of("target").expect("bad target :("));
      } else {
        make(" ");
      }
    };

    // all jobs done
    println!("--++--");
    log::trace!("shedctl finished without drop");
    Ok(())
  }
}

impl Default for ShedCli {
  fn default() -> Self {
    ShedCli::new()
  }
}

/// Global Args used by shedctl
pub struct ShedArgs(pub Vec<Arg>);

impl ShedArgs {
  pub fn new() -> Self {
    let mut args = vec![
      ShedArgs::config_arg(),
      ShedArgs::package_arg(),
      ShedArgs::registry_arg(),
    ];
    for i in ShedArgs::network_args() {
      args.push(i)
    }
    ShedArgs(args)
  }

  pub fn config_arg() -> Arg {
    Arg::with_name("config")
      .short("c")
      .long("config")
      .takes_value(true)
      .value_name("RON_FILE")
      .help("Specifies the config.ron file to use.")
  }
  pub fn network_args() -> Vec<Arg> {
    let host_arg = Arg::with_name("host")
      .short("h")
      .help("specify a network host address");
    let port_arg = Arg::with_name("port").short("p").help("specify a port");
    let mut net_args = vec![host_arg, port_arg];
    net_args
  }
  pub fn package_arg() -> Arg {
    Arg::with_name("package")
  }
  pub fn registry_arg() -> Arg {
    Arg::with_name("registry")
  }
}
