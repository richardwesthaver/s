use rlib::util::cli::{App, AppSettings, Arg};
pub fn build_cli() -> App<'static> {
  App::new("shed")
    .author("ellis <ellis@rwest.io>")
    .about("shed ← tool≍⌜⍋box")
    .setting(AppSettings::ColorAuto)
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::DisableVersionForSubcommands)
    .setting(AppSettings::TrailingVarArg)
    .setting(AppSettings::ArgRequiredElseHelp)
    .arg(
      Arg::new("config")
        .short('c')
        .long("config")
        .value_name("RON|JSON|BIN")
        .about("override configuration values")
        .takes_value(true)
        .global(true),
    )
    .subcommands(vec![
      App::new("init")
        .about("initialize the shed")
        .arg(Arg::new("path").takes_value(true).default_value("."))
        .arg(Arg::new("db").short('d').long("db"))
        .arg(
          Arg::new("fmt")
            .takes_value(true)
            .short('f')
            .about("config format")
            .possible_values(&["json", "ron", "bin"]),
        ),
      App::new("edit")
        .alias("e")
        .about("edit all the things")
        .arg(Arg::new("input").takes_value(true).default_value(".")),
      App::new("krypt")
        .alias("k")
        .about("blackbox")
        .arg(Arg::new("input").takes_value(true).default_value(".")),
      App::new("clean")
        .alias("c")
        .about("clean stuff up")
        .arg(Arg::new("input").takes_value(true).default_value(".")),
      App::new("status")
        .alias("s")
        .about("print basic info")
        .arg(Arg::new("input"))
        .arg(Arg::new("sys").long("sys").short('s').about("system info"))
        .arg(Arg::new("ip").long("ip").short('i').about("my ip"))
        .arg(Arg::new("usb").long("usb").short('u').about("usb devices"))
        .arg(
          Arg::new("midi")
            .long("midi")
            .short('m')
            .about("midi devices"),
        )
        .arg(Arg::new("weather").short('w').about("weather report"))
        .arg(
          Arg::new("remote")
            .short('r')
            .about("query remote for changes"),
        ),
      App::new("pack")
        .about("create packages from file or directory")
        .arg(Arg::new("input").takes_value(true))
        .arg(Arg::new("output").takes_value(true).default_value(".")),
      App::new("unpack")
        .about("unpack .zst or .tz files")
        .arg(Arg::new("input").takes_value(true))
        .arg(
          Arg::new("output")
            .takes_value(true)
            .default_value(".")
            .required(false),
        )
        .arg(
          Arg::new("replace")
            .short('r')
            .about("consume input package"),
        ),
      App::new("pull").about("fetch resources").arg(
        Arg::new("input")
          .takes_value(true)
          .about("parent to pull from"),
      ),
      App::new("push")
        .about("commit changes to upstream")
        .arg(Arg::new("to").takes_value(true).about("parent to push to")),
      App::new("store").about("shared block storage"),
      App::new("stash").about("local storage"),
      App::new("serve")
        .about("network services")
        .arg(
          Arg::new("package")
            .takes_value(true)
            .multiple_values(true)
            .short('p')
            .about("specify packages to serve"),
        )
        .arg(
          Arg::new("engine")
            .takes_value(true)
            .possible_values(&["hg", "dm", "ftp"])
            .about("network backend"),
        ),
      App::new("build")
        .alias("b")
        .about("build scripts")
        .arg(Arg::new("target").takes_value(true).multiple_values(true))
        .arg(
          Arg::new("pkg")
            .short('p')
            .takes_value(true)
            .multiple_values(true)
            .about("package to build"),
        ),
      App::new("x")
        .about("do things with runtimes")
        .arg(
          Arg::new("repl")
            .takes_value(true)
            .default_value("dmc")
            .possible_values(&["dmc", "py", "bqn", "k", "apl", "erl", "lua"]),
        )
        .arg(
          Arg::new("command")
            .takes_value(true)
            .multiple_values(true)
            .short('x')
            .about("execute a command"),
        )
        .arg(
          Arg::new("module")
            .takes_value(true)
            .multiple_values(true)
            .short('m')
            .about("execute a module"),
        )
        .arg(
          Arg::new("script")
            .takes_value(true)
            .alias("file")
            .short('s')
            .short_alias('f')
            .about("execute a script"),
        )
        .arg(
          Arg::new("interpreter")
            .takes_value(true)
            .short('i')
            .about("use the specified interpreter (dialect)"),
        ),
    ])
}
