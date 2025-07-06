use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::modules::command::run_command;

const LAST_UPDATE_FILE: &str = "/tmp/inti-last-apt-update";

fn should_update() -> Result<bool> {
    if !Path::new(LAST_UPDATE_FILE).exists() {
        return Ok(true);
    }
    let last_update_str = fs::read_to_string(LAST_UPDATE_FILE)?;
    let last_update_time = last_update_str.parse::<DateTime<Utc>>()?;
    let now = Utc::now();
    if now.signed_duration_since(last_update_time) > Duration::days(1) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn record_update_time() -> Result<()> {
    let now = Utc::now();
    fs::write(LAST_UPDATE_FILE, now.to_rfc3339())?;
    Ok(())
}

fn is_package_installed(package: &str) -> Result<bool> {
    let output = Command::new("dpkg-query")
        .arg("-W")
        .arg("-f='${Status}'")
        .arg(package)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("install ok installed"))
}

pub fn apt_update(force: bool, verbose: bool) -> Result<()> {
    if force || should_update()? {
        println!("Running apt-get update...");
        run_command(Command::new("sudo").arg("apt-get").arg("update"), verbose)?;
        record_update_time()?;
    } else {
        println!("Skipping apt-get update as it was run recently.");
    }
    Ok(())
}

pub fn apt_install(package: &str, verbose: bool) -> Result<()> {
    if !is_package_installed(package)? {
        println!("Installing package: {}", package);
        run_command(
            Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("-y")
                .arg(package),
            verbose,
        )?;
    } else {
        println!("Package {} is already installed.", package);
    }
    Ok(())
}
