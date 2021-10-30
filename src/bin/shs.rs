/// bin/shs.rs --- shed-server
use rlib::{ctx, logger::flexi, kala::Result};

#[ctx::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}