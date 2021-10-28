/// bin/shd.rs --- shed-daemon
use rlib::{ctx, logger::flexi, kala::Result};
#[ctx::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}
