use crate::application::ports::screen_repository::ScreenRepository;
use crate::domain::entities::screen_session::{ScreenSession, SessionStatus};
use crate::infrastructure::adapters::screen_command::ScreenCommand;
use std::error::Error;

/// Concrete implementation of ScreenRepository that uses the screen command
pub struct ScreenRepositoryImpl {
    command: ScreenCommand,
}

impl ScreenRepositoryImpl {
    pub fn new(command: ScreenCommand) -> Self {
        Self { command }
    }

    /// Parse the output of 'screen -ls' command
    fn parse_screen_output(&self, output: &str) -> Vec<ScreenSession> {
        let lines: Vec<&str> = output.lines().collect();
        
        // If output is empty or has only one line, there are no sessions
        if lines.len() <= 1 {
            return Vec::new();
        }

        let mut sessions = Vec::new();

        // Process each screen session line
        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            
            // Skip the summary line at the end
            if trimmed.contains("Socket") || trimmed.is_empty() {
                continue;
            }
            
            // Parse the screen session information
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            // Extract details
            let session_info = parts[0];
            let mut status = SessionStatus::Unknown;
            let mut date_time = "Unknown".to_string();

            // Find date/time and status in the line
            for (i, part) in parts.iter().enumerate() {
                if *part == "(Detached)" {
                    status = SessionStatus::Detached;
                } else if *part == "(Attached)" {
                    status = SessionStatus::Attached;
                }
                
                // Try to extract the datetime, which is typically in parentheses
                if part.starts_with("(") && !part.contains("Detached") && !part.contains("Attached") {
                    // Join parts that might form the date time
                    if i + 1 < parts.len() {
                        date_time = parts[i..=i+1].join(" ").trim_matches(|c| c == '(' || c == ')').to_string();
                    } else {
                        date_time = part.trim_matches(|c| c == '(' || c == ')').to_string();
                    }
                }
            }

            // Split session_info to get ID and name
            let session_parts: Vec<&str> = session_info.split('.').collect();
            let id = if !session_parts.is_empty() { session_parts[0].to_string() } else { "Unknown".to_string() };
            let name = if session_parts.len() > 1 { session_parts[1..].join(".") } else { "Unknown".to_string() };

            sessions.push(ScreenSession::new(id, name, date_time, status));
        }

        sessions
    }
}

impl ScreenRepository for ScreenRepositoryImpl {
    fn list_sessions(&self) -> Result<Vec<ScreenSession>, Box<dyn Error>> {
        let output = self.command.execute(&["-ls"])?;
        Ok(self.parse_screen_output(&output))
    }

    fn create_session(&self, name: &str, command: Option<&str>) -> Result<(), Box<dyn Error>> {
        let mut args = vec!["-S", name];
        
        if let Some(cmd) = command {
            args.push("-c");
            args.push(cmd);
        }
        
        // Use interactive mode for creating sessions
        self.command.execute_interactive(&args)?;
        Ok(())
    }

    fn attach_session(&self, id: &str) -> Result<(), Box<dyn Error>> {
        // Use interactive mode for attaching to sessions
        self.command.execute_interactive(&["-r", id])?;
        Ok(())
    }

    fn detach_session(&self) -> Result<(), Box<dyn Error>> {
        // Screen detach key sequence is typically Ctrl+A d
        // This can't be directly executed from code
        // In a real app, we'd need a more complex approach
        Err("Detach operation requires user to press Ctrl+A d in screen session".into())
    }

    fn kill_session(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.command.execute(&["-X", "-S", id, "quit"])?;
        Ok(())
    }
    
    fn detach_and_reattach_session(&self, id: &str) -> Result<(), Box<dyn Error>> {
        // Use interactive mode for detaching and reattaching
        self.command.execute_interactive(&["-d", "-r", id])?;
        Ok(())
    }
} 