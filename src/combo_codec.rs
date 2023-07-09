use std::io;

use bytes::{BytesMut};
use tokio_util::codec::{LengthDelimitedCodec, Decoder, Encoder};

use crate::event_codec::EventCodec;

use super::event_codec::EventBusMessage;
use super::{TypeDecoder, TypeEncoder};
use super::json_codec::JsonCodec;

/// ComboCodec以及它的head分量都实现了tokio的codec trait。
/// 
/// 这里实际上是将head codec和后续定义的其它codec组合构成流水线工作模式，从而构成最后的应用数据流
/// 
/// 但同时，这个ComboCodec本身也是符合tokio的codec trait的一个编码器。
#[derive(Debug, Clone)]
pub struct ComboCodec {
  head: LengthDelimitedCodec,
  json_codec: JsonCodec,
  event_codec: EventCodec,
}

impl ComboCodec {
  pub fn new() -> Self {
    Self {
      head: LengthDelimitedCodec::new(),
      json_codec: JsonCodec::new(),
      event_codec: EventCodec::new(),
    }
  }
}

/// 流水线组合后的处理逻辑，实际依然满足tokio的Decoder的要求。
impl Decoder for ComboCodec { 
  type Item = EventBusMessage;  // 说明EventBusMessage是这个流水线实际构造的应用数据结构
  type Error = io::Error;  // 考虑到主要的流是来自于网络通信，采用io::Error作为错误承载类。并非强制选择。

  /// 流水线处理逻辑，这里head和后续的处理逻辑实际上是不一样的。
  /// 
  /// try_decode是TypeEncoder trait中的方法，其作用是将Option分支的None沿流水线传递，
  /// 最后由tokio的trait按缺省方式处理。
  /// 
  /// 这样设计的理由是为了适配tokio的流处理流程，保证组合构成后的ComboCodec和tokio正常匹配工作。
  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    self.head.decode(src)
      .and_then(|v| self.json_codec.try_decode(&v))
      .and_then(|v| self.event_codec.try_decode(&v))
  }
}

/// 流水线组合后的处理逻辑，实际依然满足tokio的Encoder的要求。
impl Encoder<EventBusMessage> for ComboCodec {
    type Error = io::Error;

    fn encode(&mut self, item: EventBusMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
      self.event_codec.encode(item)
        .and_then(|v| self.json_codec.encode(v))
        .and_then(|v| self.head.encode(v, dst))
    }
}