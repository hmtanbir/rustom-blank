use crate::config::AppConfig;
use crate::services::DynQueueService;

/// Struct containing shared application dependencies.
#[derive(Clone)]
pub struct AppState {
    /// Publisher service for pushing jobs to RabbitMQ.
    pub queue_publisher: DynQueueService,
    /// Environment variables configuration.
    pub config: AppConfig,
}
