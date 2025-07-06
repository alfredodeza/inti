use anyhow::Result;
use std::process::Command;

pub fn run_command(command: &mut Command, verbose: bool) -> Result<()> {
    if verbose {
        println!("Executing command: {:?}", command);
    }
    let output = command.output()?;
    if verbose {
        if !output.stdout.is_empty() {
            println!("stdout: {}\n", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            println!("stderr: {}\n", String::from_utf8_lossy(&output.stderr));
        }
    }
    if !output.status.success() {
        anyhow::bail!("Command failed with status {}: {:?}", output.status, command);
    }
    Ok(())
}

pub fn check_command(command_str: &str, verbose: bool) -> Result<bool> {
    if verbose {
        println!("Checking 'unless' condition: {}", command_str);
    }
    let mut command = Command::new("sh");
    command.arg("-c").arg(command_str);
    let status = command.status()?;
    if verbose {
        println!("'unless' command finished with status: {}", status);
    }
    Ok(status.success())
}