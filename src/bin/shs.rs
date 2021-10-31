/// bin/shs.rs --- shed-server
use rlib::{ctx, kala::Result, logger::flexi};

#[ctx::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}
