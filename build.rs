use rlib::util::{bs::version::generate_cargo_keys,
                 {cli::comp_gen::{Bash, PowerShell, Zsh, generate_to}}};
use std::{io::Error, env};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
  generate_cargo_keys();

  let o = match env::var_os("OUT_DIR") {
    None => return Ok(()),
    Some(out) => out,
  };  
  let c = (&mut build_cli(), "shed", o);
  let b = generate_to::<Bash, _, _>(c.0,c.1,&c.2)?;
  let z = generate_to::<Zsh, _, _>(c.0,c.1,&c.2)?;
  let p = generate_to::<PowerShell, _, _>(c.0,c.1,c.2)?;
  println!("cargo:warning=bash completion generated: {:?}", b);
  println!("cargo:warning=zsh completion generated: {:?}", z);
  println!("cargo:warning=ps1 completion generated: {:?}", p);
  println!("cargo:rerun-if-changed=build.rs");
  Ok(())
}
