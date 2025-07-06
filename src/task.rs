use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TaskContext {
    pub verbose: bool,
    pub dry_run: bool,
    pub working_directory: String,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub success: bool,
    pub changed: bool,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub trait TaskExecutor: Send + Sync {
    fn execute(&self, context: &TaskContext) -> Result<TaskResult>;
    fn task_type(&self) -> &'static str;
    fn validate(&self) -> Result<()> {
        Ok(())
    }
    fn description(&self) -> &'static str {
        ""
    }
    fn requires_sudo(&self) -> bool {
        false
    }
    fn is_platform_supported(&self) -> bool {
        true
    }
    fn is_idempotent(&self) -> bool {
        true
    }
    fn tags(&self) -> Vec<&'static str> {
        vec![]
    }
    fn estimated_duration(&self) -> Option<u64> {
        None
    }
}
