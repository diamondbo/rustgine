use crate::config::Config as CoreConfig;

#[test]
fn config_loads_successfully() {
    let config = CoreConfig::load();
    assert!(config.is_ok(), "Config should load without error");
}
