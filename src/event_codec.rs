use std::{io, panic};

use serde_json::{json, Value};

use super::{TypeDecoder, TypeEncoder};

#[derive(Debug, Clone)]
pub enum EventBusMessage {
    Ping,
    Pong,
    Send {
        // 网络响应
        address: String,
        headers: Value,
        body: Value,
    },
    Message {
        // 网络请求
        address: String,
        headers: Value,
        body: Value,
        reply_address: Option<String>,
    },
    Register(String),
    /*Err {
        failure_code: i32,
        failure_type: String,
        message: String,
        source_address: String,
    }*/
}

impl EventBusMessage {
    pub fn decode(src: Value) -> Option<Self> {
        let reply_address: Option<String> = match src["replyAddress"].as_str() {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        match src["type"].as_str().unwrap() {
            "ping" => Some(EventBusMessage::Ping),
            "pong" => Some(EventBusMessage::Pong),
            "send" => Some(EventBusMessage::Send {
                address: src["address"].as_str().unwrap().to_string(),
                headers: src["headers"].clone(),
                body: src["body"].clone(),
            }),
            "message" => Some(EventBusMessage::Message {
                address: src["address"].as_str().unwrap().to_string(),
                headers: src["headers"].clone(),
                body: src["body"].clone(),
                reply_address: reply_address,
            }),
            "register" => Some(EventBusMessage::Register(src["address"].as_str().unwrap().to_string())),
            _ => {
                log::error!("未定义解释的消息: {}", src);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventCodec;

impl EventCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl TypeDecoder<EventBusMessage> for EventCodec {
    type Source = serde_json::Value;
    type Error = io::Error;

    fn decode(&mut self, src: &Self::Source) -> Result<Option<EventBusMessage>, Self::Error> {
        panic::catch_unwind(|| EventBusMessage::decode(src.clone()))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, format!("消息数据不完全：{src}")))
    }
}

impl TypeEncoder<EventBusMessage> for EventCodec {
    type Destination = Value;
    type Error = io::Error;

    fn encode(&mut self, item: EventBusMessage) -> Result<Self::Destination, Self::Error> {
        let res = match item {
            EventBusMessage::Ping => {
                json!({
                    "type":"ping"
                })
            }
            EventBusMessage::Pong => {
                json!({
                    "type":"pong"
                })
            }
            EventBusMessage::Send {
                address,
                headers,
                body,
            } => {
                json!({
                    "type": "send",
                    "address": address,
                    "headers": headers,
                    "body": body
                })
            }
            EventBusMessage::Register(address) => {
                json!({
                    "type": "register",
                    "address": address
                })
            }
            EventBusMessage::Message {
                address,
                headers: _,
                body,
                reply_address,
            } => {
                json!({
                    "type": "message",
                    "address": address,
                    "body": body,
                    "replyAddress": reply_address
                })
            }

            /*EventMessage::Err { failure_code, failure_type, message, source_address } => {
                json!({
                    "type": "err",
                    "failureCode": failure_code,
                    "failureType": failure_type,
                    "message": message,
                    "sourceAddress": source_address,
                })
            }*/
        };
        Ok(res)
    }
}
