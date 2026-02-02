use crate::{config::Config, trace::init_tracing};
use tracing::info;

#[test]
fn tracing_initializes() {
    let config = Config::load().unwrap();
    init_tracing(&config.environment);
    info!("tracing initialized");
}
