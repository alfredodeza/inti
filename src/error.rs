use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task validation failed: {0}")]
    Validation(String),
    #[error("Task execution failed: {0}")]
    Execution(String),
    #[error("Task not found: {0}")]
    NotFound(String),
    #[error("Invalid task configuration: {0}")]
    InvalidConfiguration(String),
}