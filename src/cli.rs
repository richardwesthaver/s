use util::cli::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "ellis")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  #[clap(short, long, default_value = option_env!("SHED_CFG").unwrap_or("./cfg.ron"))]
  pub config: String,
  #[clap(subcommand)]
  pub subcmd: Option<SubCommand>,
  pub input: Option<String>,
}

#[derive(Clap)]
pub enum SubCommand {
  /// bootstrap a shed
  Init(Init),
  /// build package
  Pack(Pack),
  /// extract package
  Unpack(Unpack),
  /// report current status
  Status(Status),
  /// pull changesets
  Pull(Pull),
  /// persistent storage
  Store(Store),
  /// temporary storage
  Stash(Stash),
  /// publish documentation
  Publish(Publish),
  /// host network services
  Serve(Serve),
}

#[derive(Clap)]
pub struct Init {
  #[clap(default_value = ".")]
  pub input: String,
}

#[derive(Clap)]
pub struct Pack {
  pub input: String,
  #[clap(default_value = ".")]
  pub output: String,
}

#[derive(Clap)]
pub struct Unpack {
  pub input: String,
  #[clap(default_value = ".")]
  pub output: String,
  #[clap(short, long)]
  pub replace: bool,
}

#[derive(Clap)]
pub struct Status {
  #[clap(short, long, default_value = "tree")]
  pub view: String,
}
#[derive(Clap)]
pub struct Pull {
  pub parent: Option<String>,
}

#[derive(Clap)]
pub struct Stash {
}

#[derive(Clap)]
pub struct Store {
}

#[derive(Clap)]
pub struct Publish {
  #[clap(short, long, default_value = "current")]
  pub packages: Vec<String>,
}

#[derive(Clap)]
pub struct Serve {
  #[clap(short, long, default_value = "hg")]
  pub ty: String,
  #[clap(short, long, default_value = "current")]
  pub packages: Option<Vec<String>>,
}
