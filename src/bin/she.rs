//! bin/she.rs --- Emacs wrapper
use rlib::kala::cmd::shell::{emacs, emacsclient};

#[tokio::main]
async fn main() {
  let int = std::env::args().nth(1);
  if let Some(i) = int {
    emacsclient(vec!["--socket-name=she", "--eval", &i])
      .await
      .expect("failed to execute.");
  } else {
    emacs(vec!["--fg-daemon=she", "--eval", "(shed-cmd-server-start)"])
      .await
      .expect("failed to start.");
  }
}
