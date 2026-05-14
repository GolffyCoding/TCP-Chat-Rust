use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn init_json_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}