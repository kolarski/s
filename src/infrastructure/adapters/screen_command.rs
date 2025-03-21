use std::error::Error;
use std::process::{Command, Stdio};

/// Custom error type for screen command execution
#[derive(Debug)]
pub enum ScreenCommandError {
    NoSessionsAvailable,
    ExecutionError(String),
    Other(Box<dyn Error>),
}

impl std::fmt::Display for ScreenCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScreenCommandError::NoSessionsAvailable => write!(f, "No screen sessions available"),
            ScreenCommandError::ExecutionError(err) => write!(f, "Error executing screen command: {}", err),
            ScreenCommandError::Other(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ScreenCommandError {}

/// Adapter for interacting with the Linux screen command
pub struct ScreenCommand;

impl ScreenCommand {
    pub fn new() -> Self {
        Self {}
    }

    /// Execute the screen command with the given arguments and return output
    pub fn execute(&self, args: &[&str]) -> Result<String, Box<dyn Error>> {
        let output = Command::new("screen")
            .args(args)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Special case for listing sessions: screen returns an error when no sessions exist
        if args.contains(&"-ls") && !output.status.success() {
            // Check various error conditions that indicate no sessions
            if stderr.contains("No Sockets found") || 
               stderr.contains("No screen session") || 
               stderr.is_empty() {
                return Err(Box::new(ScreenCommandError::NoSessionsAvailable));
            }
        }

        // Also check stdout for indications of no sessions (some versions output this differently)
        if args.contains(&"-ls") && (
            stdout.contains("No Sockets found") || 
            stdout.contains("No screen session") ||
            (stdout.trim().is_empty() && stderr.trim().is_empty())
        ) {
            return Err(Box::new(ScreenCommandError::NoSessionsAvailable));
        }

        if output.status.success() {
            Ok(stdout)
        } else {
            // If stderr is empty, provide a more helpful error message
            if stderr.trim().is_empty() {
                Err(Box::new(ScreenCommandError::ExecutionError(
                    "Screen command failed but didn't provide an error message. This often happens when no screen sessions exist.".to_string()
                )))
            } else {
                Err(Box::new(ScreenCommandError::ExecutionError(stderr)))
            }
        }
    }
    
    /// Execute the screen command in interactive mode (for creating/attaching sessions)
    pub fn execute_interactive(&self, args: &[&str]) -> Result<(), Box<dyn Error>> {
        let status = Command::new("screen")
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
            
        if status.success() {
            Ok(())
        } else {
            Err(Box::new(ScreenCommandError::ExecutionError(format!(
                "Screen command failed with exit code: {:?}",
                status.code()
            ))))
        }
    }
} 