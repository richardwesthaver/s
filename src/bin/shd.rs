//! bin/shd.rs --- shed-daemon
use rlib::{kala::Result, logger::flexi};

#[tokio::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}
