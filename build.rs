use rlib::util::{
  bs::version::generate_cargo_keys,
  {cli::comp_gen::{Bash,PowerShell,Zsh,generate_to}}};
use rlib::flate::{flate2::{Compression,GzBuilder}, tar};
use std::{env,io::Error,path::Path,fs,ffi::OsStr};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
  generate_cargo_keys();

  let oc = match env::var_os("OUT_DIR") {
    None => return Ok(()),
    Some(out) => out,
  };  
  let od = Path::new(&oc).join("man.tgz");
  let c = (&mut build_cli(), "shed", &oc);
  let d = fs::File::create(Path::new(&od)).unwrap();
  let e = GzBuilder::new()
    .filename("man.tar")
    .write(d, Compression::best());

  let mut ar = tar::Builder::new(e);
  ar.mode(tar::HeaderMode::Deterministic);

  let mut add_files = |dir, extension| {
    let mut files = fs::read_dir(dir)
      .unwrap()
      .map(|e| e.unwrap().path())
      .collect::<Vec<_>>();
    files.sort();
    for path in files {
      if path.extension() != Some(extension) {
        continue;
      }
      println!("cargo:rerun-if-changed={}", path.display());
      ar.append_path_with_name(&path, path.file_name().unwrap())
        .unwrap();
    }
  };  

  add_files(Path::new("docs/man"), OsStr::new("txt"));

  ar.into_inner().unwrap().finish().unwrap();

  generate_to::<Bash, _, _>(c.0,c.1,c.2)?;
  generate_to::<Zsh, _, _>(c.0,c.1,c.2)?;
  generate_to::<PowerShell, _, _>(c.0,c.1,c.2)?;
  
  println!("cargo:rerun-if-changed=build.rs");
  Ok(())
}
