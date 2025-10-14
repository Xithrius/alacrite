use color_eyre::{Result, eyre::eyre};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logging with the specified log level
pub fn init_logging(log_level: &str) -> Result<()> {
    let env_filter = EnvFilter::try_new(log_level)
        .map_err(|e| eyre!("Failed to create environment filter: {}", e))?;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();

    Ok(())
}
