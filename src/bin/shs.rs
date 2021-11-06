/// bin/shs.rs --- shed-server
use rlib::{logger::flexi, util::Result};

#[tokio::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}
