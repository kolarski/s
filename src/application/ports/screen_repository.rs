use crate::domain::entities::screen_session::ScreenSession;
use std::error::Error;

/// Interface for a repository that manages screen sessions
pub trait ScreenRepository {
    /// List all screen sessions
    fn list_sessions(&self) -> Result<Vec<ScreenSession>, Box<dyn Error>>;
    
    /// Create a new screen session
    fn create_session(&self, name: &str, command: Option<&str>) -> Result<(), Box<dyn Error>>;
    
    /// Attach to an existing session
    fn attach_session(&self, id: &str) -> Result<(), Box<dyn Error>>;
    
    /// Detach from the current session
    fn detach_session(&self) -> Result<(), Box<dyn Error>>;
    
    /// Kill/terminate a session
    fn kill_session(&self, id: &str) -> Result<(), Box<dyn Error>>;
    
    /// Detach (if needed) and reattach to a session that might already be attached
    fn detach_and_reattach_session(&self, id: &str) -> Result<(), Box<dyn Error>>;
} 