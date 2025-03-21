use std::error::Error;
use std::io::{self, Write};

/// Adapter for getting user input
pub struct UserInput;

impl UserInput {
    pub fn new() -> Self {
        Self {}
    }

    /// Prompt the user with a yes/no question and return true for yes, false for no
    /// Default answer is provided and used when the user just presses Enter
    pub fn confirm(&self, message: &str, default: bool) -> Result<bool, Box<dyn Error>> {
        let default_option = if default { "Y/n" } else { "y/N" };
        print!("{} [{}]: ", message, default_option);
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        let response = response.trim().to_lowercase();
        
        if response.is_empty() {
            return Ok(default);
        }
        
        Ok(matches!(response.as_str(), "y" | "yes"))
    }
} 