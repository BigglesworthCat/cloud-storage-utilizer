use tracing::log::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn setup_logger() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(Level::Info.as_str()));

    let file_appender = tracing_appender::rolling::never("./logs", "log.txt");
    let file_layer = tracing_subscriber::fmt::layer().with_writer(file_appender);

    let subscriber = Registry::default().with(env_filter).with(file_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber")
}
