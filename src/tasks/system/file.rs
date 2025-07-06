use crate::task::{TaskContext, TaskExecutor, TaskResult};
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileTask {
    path: String,
    state: String,
}

impl TaskExecutor for FileTask {
    fn execute(&self, context: &TaskContext) -> Result<TaskResult> {
        if self.state == "directory" {
            crate::modules::file::create_directory(&self.path, context.verbose)?;
        }
        Ok(TaskResult {
            success: true,
            changed: true, // This should be properly implemented later
            message: format!("Directory {} created", self.path),
            stdout: None,
            stderr: None,
        })
    }

    fn task_type(&self) -> &'static str {
        "file"
    }

    fn description(&self) -> &'static str {
        "Manage files and directories"
    }
}
