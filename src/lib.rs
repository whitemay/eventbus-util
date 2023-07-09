use std::io;

pub mod json_codec;
pub mod event_codec;
pub mod combo_codec;

pub trait TypeDecoder<Item> {
  type Source;
  type Error: From<io::Error>;

  fn decode(&mut self, src: &Self::Source) -> Result<Option<Item>, Self::Error>;

  fn try_decode(&mut self, src: &Option<Self::Source>) -> Result<Option<Item>, Self::Error> {
    match src {
      Some(src) => self.decode(src),
      None => Ok(None),
    }
  }
}

pub trait TypeEncoder<Item> {
  type Destination;
  type Error: From<io::Error>;
  fn encode(&mut self, item: Item) -> Result<Self::Destination, Self::Error>;
}
