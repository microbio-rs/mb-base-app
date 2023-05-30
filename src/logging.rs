use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
pub struct LogSettings {
    pub level: String,
}

impl LogSettings {
    pub fn level(&self) -> String {
        format!("mb_base_app={level},tower_http={level}", level = self.level)
    }
}

pub fn log_init(settings: &LogSettings) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| settings.level().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
