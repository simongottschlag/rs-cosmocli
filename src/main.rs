use log::*;
mod clients;
mod commands;
use clients::app::app;

fn main() {
    env_logger::init();
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(app()) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error ocurred: {}", e),
    };
}
