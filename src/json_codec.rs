use std::fmt::Debug;
use std::io;

use bytes::{BytesMut, Bytes};
use serde_json::Value;

use super::{TypeDecoder, TypeEncoder};

#[derive(Clone)]
pub struct JsonCodec;

impl JsonCodec {
    pub fn new() -> Self {
      Self{}
    }
}

impl Debug for JsonCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("JsonCodec")
    }
}

impl TypeDecoder<Value> for JsonCodec {
    type Source=BytesMut;

    type Error = io::Error;

    fn decode(&mut self, src: &Self::Source) -> Result<Option<Value>, Self::Error> {
      // 解码错误不应恢复。
      match serde_json::from_slice::<Value>(&src) {
        Ok(value) => Ok(Some(value)),
        Err(e) => Err(e.into())
      }
    }
}

impl TypeEncoder<Value> for JsonCodec {
    type Error = io::Error;
    type Destination=Bytes;

    fn encode(&mut self, item: Value) -> Result<Self::Destination, Self::Error> {
      Ok(Bytes::from(item.to_string()))
    }
}