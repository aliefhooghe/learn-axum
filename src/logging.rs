use tracing_subscriber::EnvFilter;

use crate::settings;

pub fn init(settings: &settings::Settings) {
    let filter = EnvFilter::new(
        settings
            .logging
            .as_ref()
            .map(|l| l.level.as_ref())
            .unwrap_or("INFO"),
    );

    tracing_subscriber::fmt()
        .with_level(true)
        .with_env_filter(filter)
        .init();
}
