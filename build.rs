//! build.rs --- shed build script
/*!
this script provides the 'DEMON_VERSION' variable for all builds,
which adds a Mercurial commit hash to the package version.

When 'PROFILE'='release' also generate bash, zsh, and powershell
completions.
*/

use rlib::util::{
  bs::version::generate_cargo_keys,
  cli::comp_gen::{generate_to, Bash, PowerShell, Zsh},
  Result,
};

use std::{env, fs, path};

include!("src/cli.rs");

fn main() -> Result<()> {
  generate_cargo_keys();
  let o: path::PathBuf = env!("CARGO_MANIFEST_DIR").into();
  if !o.exists() {
    fs::create_dir(&o)?;
  }
  let c = (&mut build_cli(), "shc", &o.join("etc/"));
  generate_to(Bash, c.0, c.1, c.2)?;
  generate_to(Zsh, c.0, c.1, c.2)?;
  generate_to(PowerShell, c.0, c.1, c.2)?;
  println!("cargo:rerun-if-changed=build.rs");
  Ok(())
}
