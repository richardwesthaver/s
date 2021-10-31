/// bin/shd.rs --- shed-daemon
use rlib::{ctx, kala::Result, logger::flexi};
#[ctx::main]
async fn main() -> Result<()> {
  flexi("trace")?;
  Ok(())
}
