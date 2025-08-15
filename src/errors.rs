use thiserror::Error;

#[derive(Error, Debug)]
pub enum VortexError {
    #[error("Failed to enqueue item into work queue")]
    Enqueue,
    #[error("Failed to create a new queue: {0}")]
    QueueCreation(String),
    #[error("Mesocarp feature is not enabled")]
    MesocarpNotEnabled,
    #[error("An error occurred in the Mesocarp wrapper: {0}")]
    MesocarpWrapper(String),
    #[error("Kernel with name '{0}' not found in registry")]
    KernelNotFound(String),
    #[error("An unknown error has occurred")]
    Unknown,
}
