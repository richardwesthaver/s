use clap::{AppSettings, ArgSettings, Clap};
use logger::log::info;
use shed::{Client, Config, Result, Server};
/// Shed CLI Options
#[derive(Clap, Debug)]
#[clap(name = "shed",
       author,
       about,
       version,
       setting = AppSettings::ColorAuto,
       setting = AppSettings::ColoredHelp)]
pub struct Opt {
    /// the subcommand to execute
    #[clap(subcommand)]
    cmd: Option<Cmd>,
    /// config.ron to use for this run
    #[clap(short, long, env = "SHED_CFG")]
    config: String,

    /// input files
    #[clap(short, long)]
    input: Option<Vec<String>>,

    /// stdout if no file, no logging if not present
    #[clap(long)]
    #[allow(clippy::option_option)]
    log: Option<Option<String>>,

    /// Output files
    #[clap(short, long)]
    out: Option<Vec<String>>,

    #[clap(long)]
    /// DemonId
    id: Option<String>,
}

#[derive(Clap, Debug)]
#[clap()]
pub enum Cmd {
    /// pack a Package from the Registry in tar.zst format
    Pack,
    /// unpack a tar.zst compressed archive
    Unpack,
    /// report the current shed status
    Status,
    /// pull changesets from a remote
    Pull,
    /// update local copy and refresh cache
    Update,
    /// host shed network services
    Serve,
}

#[ctx::main]
async fn main() -> Result<()> {
    logger::flexi()?;
    let opt = Opt::parse();
    println!("{:#?}", opt);

    let cfg = Config::load(&opt.config);
    info!("{:#?}", cfg);
    Ok(())
}
