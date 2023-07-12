use std::{sync::Arc, io};

use actix_rt::net::TcpStream;
use eventbus_util::{combo_codec::ComboCodec, event_codec::EventBusMessage};
use futures::{StreamExt, SinkExt};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use tokio_rustls::{TlsConnector, rustls};
use tokio_util::codec::Decoder;

/// 这里编写一个使用SSL连接的例子
#[actix_rt::main]
async fn main() -> Result<(), io::Error>{
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_threads(true)
        .init()
        .unwrap();
    log::debug!("starting example");

    // 创建SSL连接
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth(); // i guess this was previously the default?
    let connector = TlsConnector::from(Arc::new(config));

    let stream = TcpStream::connect(("127.0.0.1", 8000)).await?;
    let domain = rustls::ServerName::try_from("localhost")
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;
    let stream = connector.connect(domain, stream).await?;

    let codec = ComboCodec::new();

    let (mut tx, mut rx) = codec.framed(stream).split();

    actix_rt::spawn(async move { 
        while let Some(msg) = rx.next().await  {
            log::debug!("received {:?}", msg);
        }
    });

    tx.send(EventBusMessage::Ping).await?;
    
    // .....
    
    Ok(())
}
