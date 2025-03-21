use crate::application::ports::screen_repository::ScreenRepository;
use crate::infrastructure::adapters::screen_command::ScreenCommandError;
use crate::presentation::settings::{AppSettings, AppIcons};
use std::error::Error;

/// Use case for killing/removing a screen session
pub struct KillScreenSession<T: ScreenRepository> {
    repository: T,
    settings: AppSettings,
}

impl<T: ScreenRepository> KillScreenSession<T> {
    pub fn new(repository: T) -> Self {
        Self { 
            repository,
            settings: AppSettings::new(),
        }
    }
    
    pub fn with_settings(repository: T, settings: AppSettings) -> Self {
        Self { repository, settings }
    }

    /// Kill a screen session with the given name or ID
    pub fn execute(&self, input: &str) -> Result<(), Box<dyn Error>> {
        // Get all sessions
        let sessions_result = self.repository.list_sessions();
        
        // Handle special case: "No screen sessions available"
        let sessions = match sessions_result {
            Ok(sessions) => sessions,
            Err(err) => {
                // Check if this is the "no sessions" error
                if let Some(screen_err) = err.downcast_ref::<ScreenCommandError>() {
                    if matches!(screen_err, ScreenCommandError::NoSessionsAvailable) {
                        let info_icon = if self.settings.use_emoticons { AppIcons::info() } else { "" };
                        return Err(format!("{}No screen sessions available to kill.", info_icon).into());
                    } else {
                        // Other screen errors should be propagated
                        return Err(err);
                    }
                } else {
                    // Other errors should be propagated
                    return Err(err);
                }
            }
        };
        
        // Find the session with the given name or ID
        let mut session_id = None;
        let mut session_name = None;
        
        for session in &sessions {
            if session.name == input || session.id == input {
                session_id = Some(session.id.clone());
                session_name = Some(session.name.clone());
                break;
            }
        }
        
        // Get the name to display (use input if we didn't find a name)
        let display_name = session_name.as_deref().unwrap_or(input);
        
        // If found, kill it
        match session_id {
            Some(id) => {
                let kill_icon = if self.settings.use_emoticons { AppIcons::kill() } else { "" };
                println!("{}Killing screen session '{}'...", kill_icon, display_name);
                self.repository.kill_session(&id)?;
                let success_icon = if self.settings.use_emoticons { AppIcons::success() } else { "" };
                println!("{}Session killed successfully.", success_icon);
                Ok(())
            },
            None => {
                let error_icon = if self.settings.use_emoticons { AppIcons::error() } else { "" };
                Err(format!("{}No screen session found with name or ID '{}'", error_icon, input).into())
            },
        }
    }
} 