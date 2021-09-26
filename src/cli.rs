use rlib::util::cli::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "ellis")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  #[clap(short, long)]
  pub config: Option<String>,
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
  /// push changesets
  Push(Push),
  /// persistent storage
  Store(Store),
  /// temporary storage
  Stash(Stash),
  /// publish documentation
  Publish(Publish),
  /// host network services
  Serve(Serve),
  /// build a program or library
  Build(Build),
  /// documentation
  Meta(Meta),
  /// notes
  Note(Note),
}

#[derive(Clap)]
pub struct Init {
  #[clap(default_value = ".")]
  pub path: String,
  #[clap(short, long)]
  pub config: Option<String>,
  #[clap(short, long)]
  pub json: bool,
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
  #[clap(default_value = ".")]
  pub input: String,
  #[clap(short, long)]
  pub sys: bool,
}
#[derive(Clap)]
pub struct Pull {
  #[clap(default_value = ".")]
  pub input: String,
}

#[derive(Clap)]
pub struct Push {
  #[clap(default_value = ".")]
  pub input: String,
}

#[derive(Clap)]
pub struct Stash {
}

#[derive(Clap)]
pub struct Store {
}

#[derive(Clap)]
pub struct Build {
}

#[derive(Clap)]
pub struct Publish {
  #[clap(short, long, default_value = "current")]
  pub packages: Vec<String>,
}

#[derive(Clap)]
pub struct Serve {
  #[clap(default_value = "http")]
  pub engine: String,
  #[clap(short, long)]
  pub packages: Option<Vec<String>>,
}

#[derive(Clap)]
pub struct Meta {
  #[clap(short, long)]
  view: Option<String>,
}

#[derive(Clap)]
pub struct Note {
  #[clap(short, long)]
  view: Option<String>,
}
