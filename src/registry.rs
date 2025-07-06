use crate::error::TaskError;
use crate::task::TaskExecutor;
use anyhow::Result;
use serde_yaml::Value;
use std::collections::HashMap;

use crate::tasks::packages::apt::AptTask;
use crate::tasks::system::command::CommandTask;
use crate::tasks::system::file::FileTask;

pub type TaskFactory = fn(&serde_yaml::Value) -> Result<Box<dyn TaskExecutor>>;

pub struct TaskRegistry {
    factories: HashMap<String, TaskFactory>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        let mut factories: HashMap<String, TaskFactory> = HashMap::new();
        Self::register_package_tasks(&mut factories);
        Self::register_system_tasks(&mut factories);
        Self { factories }
    }

    fn register_package_tasks(factories: &mut HashMap<String, TaskFactory>) {
        factories.insert("apt".to_string(), |config| {
            let task: AptTask = serde_yaml::from_value(config.clone())?;
            Ok(Box::new(task))
        });
    }

    fn register_system_tasks(factories: &mut HashMap<String, TaskFactory>) {
        factories.insert("file".to_string(), |config| {
            let task: FileTask = serde_yaml::from_value(config.clone())?;
            Ok(Box::new(task))
        });
        factories.insert("command".to_string(), |config| {
            let task: CommandTask = serde_yaml::from_value(config.clone())?;
            Ok(Box::new(task))
        });
    }

    pub fn create_task(&self, task_type: &str, config: &Value) -> Result<Box<dyn TaskExecutor>> {
        let factory = self.factories.get(task_type).ok_or_else(|| {
            TaskError::NotFound(format!("Task type '{}' not found", task_type))
        })?;
        let task = factory(config)?;
        task.validate()?;
        Ok(task)
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}