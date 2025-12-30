use anyhow::{Context, Result};
use serde::Serialize;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::Config;

pub struct Display {
    python_path: String,
    display_module: String,
}

impl Display {
    pub fn new(config: &Config) -> Self {
        Self {
            python_path: config.python_interpreter(),
            display_module: config.display.display_module.clone(),
        }
    }
    
    /// Display data using Python rich module
    pub fn show<T: Serialize>(&self, display_type: &str, data: &T) -> Result<()> {
        // Serialize data to JSON
        let json_data = serde_json::to_string(data)
            .context("Failed to serialize data to JSON")?;
        
        // Spawn Python process
        let mut child = Command::new(&self.python_path)
            .arg(&self.display_module)
            .arg(display_type)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn Python display process")?;
        
        // Write JSON to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json_data.as_bytes())
                .context("Failed to write data to Python process")?;
        }
        
        // Wait for completion
        let status = child.wait()
            .context("Failed to wait for Python process")?;
        
        if !status.success() {
            anyhow::bail!("Python display process failed");
        }
        
        Ok(())
    }
}