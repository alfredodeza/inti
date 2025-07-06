use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use inti::modules;
use inti::registry::TaskRegistry;
use inti::task::TaskContext;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "inti.yaml")]
    file: String,
    #[arg(long)]
    list_tasks: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    #[serde(default = "default_parallel")]
    parallel: bool,
    #[serde(default = "default_verbose")]
    verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            parallel: default_parallel(),
            verbose: default_verbose(),
        }
    }
}

fn default_parallel() -> bool {
    false
}

fn default_verbose() -> bool {
    false
}

#[derive(Debug, Deserialize)]
struct TaskDef {
    name: String,
    unless: Option<String>,
    #[serde(flatten)]
    task: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
struct Spec {
    #[serde(default)]
    config: Config,
    tasks: Vec<TaskDef>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = TaskRegistry::new();

    if cli.list_tasks {
        println!("Available tasks:");
        for task in registry.list_tasks() {
            println!("- {}", task);
        }
        return Ok(());
    }

    let spec_file = fs::read_to_string(cli.file)?;
    let spec: Spec = serde_yaml::from_str(&spec_file)?;

    let context = TaskContext {
        verbose: spec.config.verbose,
        dry_run: false, // This can be a CLI flag later
        working_directory: ".".to_string(),
        environment: HashMap::new(),
    };

    for mut task_def in spec.tasks {
        println!("Executing task: {}", task_def.name);

        if let Some(unless_command) = &task_def.unless {
            if modules::command::check_command(unless_command, context.verbose)? {
                println!(
                    "Skipping task '{}' because 'unless' command succeeded.",
                    task_def.name
                );
                continue;
            }
        }

        if task_def.task.len() != 1 {
            anyhow::bail!(
                "Task '{}' must have exactly one task type key, but found {}: {:?}",
                task_def.name,
                task_def.task.len(),
                task_def.task.keys()
            );
        }

        let (task_type, config) = task_def.task.drain().next().unwrap();
        let task = registry.create_task(&task_type, &config)?;
        let result = task.execute(&context)?;

        if result.success {
            println!("Task '{}' completed successfully", task_def.name);
        } else {
            println!("Task '{}' failed: {}", task_def.name, result.message);
        }
    }

    Ok(())
}
