use anyhow::Result;
use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_file_operations() -> Result<()> {
    let dir = tempdir()?;
    let mut cmd = Command::cargo_bin("inti")?;
    let inti_yaml = r#"
config:
  parallel: false
  verbose: true
tasks:
  - name: "Setup project"
    file:
      path: test_dir
      state: directory
"#;
    let file_path = dir.path().join("inti.yaml");
    fs::write(&file_path, inti_yaml)?;
    cmd.arg("--file").arg(&file_path);
    cmd.current_dir(dir.path());
    cmd.assert().success();
    assert!(fs::metadata(dir.path().join("test_dir")).is_ok());
    Ok(())
}

#[test]
fn test_cli_unless_condition() -> Result<()> {
    let dir = tempdir()?;
    let mut cmd = Command::cargo_bin("inti")?;
    let inti_yaml = r#"
config:
  parallel: false
  verbose: true
tasks:
  - name: "Install Rust"
    command: "echo 'Installing Rust...'"
    unless: "./cargo --version"
"#;
    let file_path = dir.path().join("inti.yaml");
    fs::write(&file_path, inti_yaml)?;

    // Create a dummy cargo script that returns a zero exit code
    let cargo_script_path = dir.path().join("cargo");
    fs::write(&cargo_script_path, "#!/bin/sh
exit 0")?;
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(&cargo_script_path)
        .status()?;

    cmd.arg("--file").arg(&file_path);
    cmd.current_dir(dir.path());
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Skipping task 'Install Rust'"));

    // Create a dummy cargo script that returns a non-zero exit code
    fs::write(&cargo_script_path, "#!/bin/sh
exit 1")?;

    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Skipping task 'Install Rust'"));

    Ok(())
}