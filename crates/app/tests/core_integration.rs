use app::resources::Shutdown;
use rustgine_core::init_tracing;
use rustgine_core::Config;
use tracing::info;

#[test]
fn test_config_load() {
    // Should load default config or error if missing
    let config = Config::load();
    assert!(config.is_ok(), "Config should load without error");
}

#[test]
fn test_tracing_init() {
    let config = Config::load().unwrap();
    // Should not panic
    init_tracing(&config.environment);
    info!("tracing initialized");
}

#[tokio::test]
async fn test_shutdown_trigger_and_subscribe() {
    let shutdown = Shutdown::new();
    let mut rx = shutdown.subscribe();
    shutdown.trigger();
    // Should complete without hanging
    rx.recv().await;
}
