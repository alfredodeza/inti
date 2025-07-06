use crate::task::{TaskContext, TaskExecutor, TaskResult};
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AptTask {
    update: Option<serde_yaml::Value>,
    package: Option<String>,
    packages: Option<Vec<String>>,
}

impl TaskExecutor for AptTask {
    fn execute(&self, context: &TaskContext) -> Result<TaskResult> {
        if let Some(update) = self.update.clone() {
            let force = match update {
                serde_yaml::Value::String(s) => s == "force",
                serde_yaml::Value::Bool(b) => b,
                _ => false,
            };
            crate::modules::apt::apt_update(force, context.verbose)?;
        }
        if let Some(package) = &self.package {
            crate::modules::apt::apt_install(package, context.verbose)?;
        }
        if let Some(packages) = &self.packages {
            for package in packages {
                crate::modules::apt::apt_install(package, context.verbose)?;
            }
        }
        Ok(TaskResult {
            success: true,
            changed: true, // This should be properly implemented later
            message: "Apt task executed".to_string(),
            stdout: None,
            stderr: None,
        })
    }

    fn task_type(&self) -> &'static str {
        "apt"
    }

    fn description(&self) -> &'static str {
        "Manage APT packages"
    }

    fn requires_sudo(&self) -> bool {
        true
    }
}
