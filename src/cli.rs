use rlib::util::cli::{AppSettings, App, Arg};
pub fn build_cli() -> App<'static> {
  App::new("shed")
    .author("ellis <ellis@rwest.io>")
    .about("shed ← tool≍⌜⍋box")
    .setting(AppSettings::ColorAuto)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::DisableVersionForSubcommands)
    .setting(AppSettings::TrailingVarArg)
    .setting(AppSettings::ArgRequiredElseHelp)
    .arg(Arg::new("config")
         .short('c').long("config")
         .value_name("RON|JSON|BIN")
         .about("override configuration values")
         .takes_value(true).global(true))
    .subcommands(
      vec![
        App::new("init").about("system: ON")
          .arg(Arg::new("path").takes_value(true).default_value("."))
          .arg(Arg::new("fmt").takes_value(true).short('f')
               .possible_values(&["json", "ron", "bin"])),
        App::new("status").about("print basic info")
          .arg(Arg::new("sys").long("sys").short('s').about("system info"))
          .arg(Arg::new("ip").long("ip").short('i').about("my ip"))
          .arg(Arg::new("usb").long("usb").short('u').about("usb devices"))
          .arg(Arg::new("midi").long("midi").short('m').about("midi devices"))
          .arg(Arg::new("weather").short('w').about("weather report")),
        App::new("pack")
          .arg(Arg::new("input").takes_value(true))
          .arg(Arg::new("output").takes_value(true).default_value(".")),
        App::new("unpack")
          .arg(Arg::new("input").takes_value(true))
          .arg(Arg::new("output").takes_value(true).default_value("."))
          .arg(Arg::new("replace").short('r').about("consume input package")),
        App::new("pull").about("fetch remote changes")
          .arg(Arg::new("from").takes_value(true).about("parent to pull from")),
        App::new("push").about("commit changes to upstream")
          .arg(Arg::new("to").takes_value(true).about("parent to push to")),
        App::new("store").about("shared block storage"),
        App::new("stash").about("local storage"),
        App::new("serve").about("network hosting for shed modules")
          .arg(Arg::new("package").takes_value(true).multiple_values(true)
               .short('p').about("specify packages to serve"))
          .arg(Arg::new("engine").takes_value(true)
               .possible_values(&["hg","dm", "ftp"]).about("network backend")),
        App::new("build").about("build scripts"),
        App::new("x").about("do things with runtimes")
          .arg(Arg::new("repl").takes_value(true).default_value("dmc")
               .possible_values(&["dmc", "py", "bqn", "k", "apl", "erl"]))
          .arg(Arg::new("command").takes_value(true).multiple_values(true)
               .short('x').about("execute a command"))
          .arg(Arg::new("module").takes_value(true).multiple_values(true)
               .short('m').about("execute a module"))
          .arg(Arg::new("script").takes_value(true)
               .alias("file").short('s').short_alias('f')
               .about("execute a script")),
      ])}
