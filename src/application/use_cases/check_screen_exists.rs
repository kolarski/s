use crate::application::ports::screen_repository::ScreenRepository;
use crate::domain::entities::screen_session::SessionStatus;
use std::error::Error;

/// Use case for checking if a screen session exists
pub struct CheckScreenExists<T: ScreenRepository> {
    repository: T,
}

impl<T: ScreenRepository> CheckScreenExists<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    /// Check if a screen with the given name exists and return its status if it does
    pub fn execute(&self, name: &str) -> Result<Option<SessionStatus>, Box<dyn Error>> {
        let sessions = self.repository.list_sessions()?;
        
        // Look for a session with the given name
        for session in sessions {
            if session.name == name {
                return Ok(Some(session.status));
            }
        }
        
        // No session with that name found
        Ok(None)
    }
} 