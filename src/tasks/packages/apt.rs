use crate::task::{TaskContext, TaskExecutor, TaskResult};
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct AptTask {
    update: Option<serde_yaml::Value>,
    package: Option<String>,
    packages: Option<Vec<String>>,
}

impl AptTask {
    fn should_run_apt_update(timestamp_file: &str) -> Result<bool> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if let Ok(metadata) = fs::metadata(timestamp_file) {
            if let Ok(modified) = metadata.modified() {
                let modified_secs = modified.duration_since(UNIX_EPOCH)?.as_secs();
                // 3 hours = 3 * 60 * 60 = 10800 seconds
                if now - modified_secs < 10800 {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn update_timestamp(timestamp_file: &str) -> Result<()> {
        fs::write(timestamp_file, "")?;
        Ok(())
    }
}

impl TaskExecutor for AptTask {
    fn execute(&self, context: &TaskContext) -> Result<TaskResult> {
        let timestamp_file_path = "/tmp/inti-apt-update-last-ran";

        if let Some(update) = self.update.clone() {
            let force = match update {
                serde_yaml::Value::String(s) => s == "force",
                serde_yaml::Value::Bool(b) => b,
                _ => false,
            };

            if force || AptTask::should_run_apt_update(timestamp_file_path)? {
                crate::modules::apt::apt_update(force, context.verbose)?;
                AptTask::update_timestamp(timestamp_file_path)?;
            } else {
                println!("Skipping apt update: last run less than 3 hours ago.");
            }
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

#[cfg(test)]
mod tests {
    use super::AptTask;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    use tempfile::NamedTempFile;

    #[test]
    fn test_should_run_apt_update_no_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        // Ensure the file doesn't exist for this test
        fs::remove_file(path).ok();

        assert!(AptTask::should_run_apt_update(path).unwrap());
    }

    #[test]
    fn test_should_run_apt_update_recent_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        // File is just created, so it's recent
        assert!(!AptTask::should_run_apt_update(path).unwrap());
    }

    #[test]
    fn test_should_run_apt_update_old_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Set the modification time to be older than 3 hours
        let old_time = SystemTime::now() - Duration::from_secs(10800 + 60); // 3 hours + 1 minute
        filetime::set_file_mtime(path, filetime::FileTime::from_system_time(old_time)).unwrap();

        assert!(AptTask::should_run_apt_update(path).unwrap());
    }

    #[test]
    fn test_update_timestamp() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        fs::remove_file(path).ok(); // Ensure it doesn't exist initially

        assert!(!fs::metadata(path).is_ok()); // Should not exist

        AptTask::update_timestamp(path).unwrap();

        assert!(fs::metadata(path).is_ok()); // Should exist now
    }
}

