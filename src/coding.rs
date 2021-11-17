//! coding.rs --- Shed codecs
/*!
The types in this module implement the 'Encoder and Decoder' traits
used for working with 'Framed' interfaces.
*/
use bytes::{BufMut, Bytes, BytesMut};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Codec;

impl Codec {
  pub fn new() -> Self {
    Codec {}
  }
}

impl Decoder for Codec {
  type Item = BytesMut;
  type Error = io::Error;

  fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
    if !buf.is_empty() {
      let len = buf.len();
      Ok(Some(buf.split_to(len)))
    } else {
      Ok(None)
    }
  }
}

impl Encoder<Bytes> for Codec {
  type Error = io::Error;

  fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
    buf.reserve(data.len());
    buf.put(data);
    Ok(())
  }
}
