use crate::task::{TaskContext, TaskExecutor, TaskResult};
use anyhow::Result;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct CommandTask {
    command: String,
}

impl TaskExecutor for CommandTask {
    fn execute(&self, context: &TaskContext) -> Result<TaskResult> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&self.command);
        crate::modules::command::run_command(&mut cmd, context.verbose)?;
        Ok(TaskResult {
            success: true,
            changed: true, // This should be properly implemented later
            message: format!("Command '{}' executed", self.command),
            stdout: None,
            stderr: None,
        })
    }

    fn task_type(&self) -> &'static str {
        "command"
    }

    fn description(&self) -> &'static str {
        "Execute a shell command"
    }
}
