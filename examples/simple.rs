use log::LevelFilter;
use simple_logger::SimpleLogger;



#[actix_rt::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_threads(true)
        .init()
        .unwrap();
    log::debug!("starting example");

    
}
