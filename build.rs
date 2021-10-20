use rlib::util::{
  bs::version::generate_cargo_keys,
  cli::comp_gen::{generate_to, Bash, PowerShell, Zsh},
  Result,
};
use std::{env::var, path::Path};
include!("src/cli.rs");

fn main() -> Result<()> {
  generate_cargo_keys();

  let o = var("OUT_DIR")?;
  let c = (&mut build_cli(), "shed", &o);

  generate_to(Bash, c.0, c.1, c.2)?;
  generate_to(Zsh, c.0, c.1, c.2)?;
  generate_to(PowerShell, c.0, c.1, c.2)?;

  if var("PROFILE")?.eq("release") {
    std::fs::copy(
      Path::new(&c.2).join("shed.bash"),
      "/usr/share/bash-completion/completions/shed.bash",
    )?;
  }

  println!("cargo:rerun-if-changed=build.rs");
  Ok(())
}
