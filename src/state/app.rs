use std::sync::Arc;

use super::config::Config;

pub type App = Arc<AppState>;

/// Contains all the application state and manages application state changes.
#[derive(Debug)]
pub struct AppState {
    config: Config,
}

impl AppState {
    /// Create a new application state with the default configuration.
    pub fn new() -> App {
        Arc::new(Self {
            config: Config::default(),
        })
    }

    /// The application configuration.
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// Called when the server is shutting down.
    #[expect(clippy::unused_async)]
    pub async fn close(&self) {}
}
