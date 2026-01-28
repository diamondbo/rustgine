use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing(env: &str) {
    let default = match env {
        "dev" => "debug",
        "staging" => "info",
        "prod" => "warn",
        _ => "info",
    };

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default));

    fmt().with_env_filter(filter).init();
}
