use crate::application::ports::screen_repository::ScreenRepository;
use crate::domain::entities::screen_session::ScreenSession;
use std::error::Error;

/// Use case for listing all screen sessions
pub struct ListScreenSessions<R: ScreenRepository> {
    repository: R,
}

impl<R: ScreenRepository> ListScreenSessions<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Execute the use case and return all screen sessions
    pub fn execute(&self) -> Result<Vec<ScreenSession>, Box<dyn Error>> {
        self.repository.list_sessions()
    }
} 