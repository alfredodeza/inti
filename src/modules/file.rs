use anyhow::Result;
use std::fs;

pub fn create_directory(path: &str, verbose: bool) -> Result<()> {
    if verbose {
        println!("Creating directory: {}", path);
    }
    fs::create_dir_all(path)?;
    Ok(())
}
