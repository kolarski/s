use crate::application::ports::screen_repository::ScreenRepository;
use crate::domain::entities::screen_session::SessionStatus;
use crate::infrastructure::adapters::screen_command::ScreenCommandError;
use crate::infrastructure::adapters::user_input::UserInput;
use crate::presentation::settings::{AppSettings, AppIcons};
use std::error::Error;

/// Use case for handling the command "s example" which checks if a screen exists
/// and either connects to it or prompts to create a new one
pub struct HandleScreenCommand<T: ScreenRepository> {
    repository: T,
    user_input: UserInput,
    settings: AppSettings,
}

impl<T: ScreenRepository> HandleScreenCommand<T> {
    pub fn new(repository: T, user_input: UserInput) -> Self {
        Self { 
            repository, 
            user_input,
            settings: AppSettings::new(),
        }
    }
    
    pub fn with_settings(repository: T, user_input: UserInput, settings: AppSettings) -> Self {
        Self { repository, user_input, settings }
    }

    pub fn execute(&self, input: &str) -> Result<(), Box<dyn Error>> {
        // First check if a session with this name or ID exists
        let sessions_result = self.repository.list_sessions();
        
        // Handle special case: "No screen sessions available" is a normal condition
        // when creating first session
        let sessions = match sessions_result {
            Ok(sessions) => sessions,
            Err(err) => {
                // Check if this is the "no sessions" error
                if let Some(screen_err) = err.downcast_ref::<ScreenCommandError>() {
                    if matches!(screen_err, ScreenCommandError::NoSessionsAvailable) {
                        // No sessions available is fine when creating a new session
                        Vec::new()
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
            
        let mut session_id = None;
        let mut session_name = None;
        let mut session_status = None;
        
        // Look for a session with the given name or ID
        for session in &sessions {
            if session.name == input || session.id == input {
                session_id = Some(session.id.clone());
                session_name = Some(session.name.clone());
                session_status = Some(session.status.clone());
                break;
            }
        }
        
        // Get the name to use for display and creation
        let display_name = session_name.as_deref().unwrap_or(input);
        
        // Handle based on the session existence and status
        match (session_id, session_status) {
            // Session not found - prompt to create
            (None, _) => {
                let create_icon = if self.settings.use_emoticons { AppIcons::create() } else { "" };
                let should_create = self.user_input.confirm(
                    &format!("{}Screen '{}' does not exist. Create and attach?", create_icon, display_name),
                    true
                )?;
                
                if should_create {
                    let screen_icon = if self.settings.use_emoticons { AppIcons::screen() } else { "" };
                    println!("{}Creating and attaching to screen '{}'...", screen_icon, display_name);
                    self.repository.create_session(display_name, None)?;
                } else {
                    let cancel_icon = if self.settings.use_emoticons { AppIcons::cancel() } else { "" };
                    println!("{}Operation cancelled.", cancel_icon);
                }
            },
            
            // Session found and detached - just attach
            (Some(id), Some(SessionStatus::Detached)) => {
                let attach_icon = if self.settings.use_emoticons { AppIcons::attach() } else { "" };
                println!("{}Attaching to existing detached screen '{}'...", attach_icon, display_name);
                self.repository.attach_session(&id)?;
            },
            
            // Session found and already attached - prompt to detach and reattach
            (Some(id), Some(SessionStatus::Attached)) => {
                let warning_icon = if self.settings.use_emoticons { AppIcons::warning() } else { "" };
                let should_reattach = self.user_input.confirm(
                    &format!("{}Screen '{}' is already attached. Detach and reattach?", warning_icon, display_name),
                    true
                )?;
                
                if should_reattach {
                    let attach_icon = if self.settings.use_emoticons { AppIcons::attach() } else { "" };
                    println!("{}Detaching and reattaching to screen '{}'...", attach_icon, display_name);
                    self.repository.detach_and_reattach_session(&id)?;
                } else {
                    let cancel_icon = if self.settings.use_emoticons { AppIcons::cancel() } else { "" };
                    println!("{}Operation cancelled.", cancel_icon);
                }
            },
            
            // Unknown status - try regular attach
            (Some(id), _) => {
                let attach_icon = if self.settings.use_emoticons { AppIcons::attach() } else { "" };
                println!("{}Attaching to screen '{}'...", attach_icon, display_name);
                self.repository.attach_session(&id)?;
            }
        }
        
        Ok(())
    }
} 