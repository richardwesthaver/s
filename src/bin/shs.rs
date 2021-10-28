/// bin/shs.rs --- shed-server
use rlib::{
  ctx, logger::flexi, net::Result
};

#[ctx::main]
async fn main() -> Result<()> {
  flexi("trace")?;
}
