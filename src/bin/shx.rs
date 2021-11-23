//! bin/shx.rs --- shed-x REPL tool
/*!
This program simply wraps a bunch of interpretted languages with a
`rustyline` interface. It has feature flags which can be configured to
match your gear, but will soon move to a runtime Config file setup.

One of the note-worthy features of this program is a pure-Rust
implementation of Python thanks to the `RustPython` crate.
*/
use rlib::{
  kala::{
    cmd::{
      repl::{apl, bqn, dyalog, erl, k, lua, shakti},
      shell::emacsclient,
    },
    dmc, python,
  },
  logger::{flexi, log},
  util::Result,
};

use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  flexi("error")?; //log to stderr
                   // store as String to match against Config
  let inter = match env::args().nth(1) {
    Some(i) => i,
    None => "dmc".to_string(),
  };
  //      "python" | "py" => python::run(|_vm|{},opt,), // need to replace opt

  // this will be replaced with lookup against cfg
  match inter.as_str() {
    "apl" => {
      apl(vec![]).await?;
    }
    "dyl" => {
      dyalog(vec!["-b"]).await?;
    }
    "erl" => {
      erl(vec![]).await?;
    }
    "k9" => {
      shakti(vec![]).await?;
    }
    "k6" => {
      k(vec![]).await?;
    }
    "elisp" | "el" => {
      emacsclient(vec!["-t", "e", "(ielm)"]).await?;
    }
    "lua" => unimplemented!(),
    "bqn" => unimplemented!(),
    "dmc" | _ => dmc::run()?,
    /* if let Some(f) = opt.values_of("script") {
     *   args.insert(0, "-f");
     *   for (x, i) in f.enumerate() {
     *     args.insert(x, i);
     *   }
     *   bqn(args).await?;
     * } else if let Some(f) = opt.values_of("command") {
     *   args.insert(0, "-p");
     *   for (x, i) in f.enumerate() {
     *     args.insert(x, i);
     *   }
     *   bqn(args).await?;
     * } else {
     *   args.insert(0, "-r");
     *   bqn(args).await?;
     * }
     *    _ => log::error!("unknown interpreter"), */
  }
  Ok(())
}
