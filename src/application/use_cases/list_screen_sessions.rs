use crate::application::ports::screen_repository::ScreenRepository;
use crate::domain::entities::screen_session::ScreenSession;
use crate::presentation::settings::AppSettings;
use std::error::Error;

/// Use case for listing all screen sessions
pub struct ListScreenSessions<R: ScreenRepository> {
    repository: R,
    settings: AppSettings,
}

impl<R: ScreenRepository> ListScreenSessions<R> {
    pub fn new(repository: R) -> Self {
        Self { 
            repository,
            settings: AppSettings::new(),
        }
    }
    
    pub fn with_settings(repository: R, settings: AppSettings) -> Self {
        Self { repository, settings }
    }

    /// Execute the use case and return all screen sessions
    pub fn execute(&self) -> Result<Vec<ScreenSession>, Box<dyn Error>> {
        self.repository.list_sessions()
    }
    
    /// Get the current settings
    pub fn get_settings(&self) -> &AppSettings {
        &self.settings
    }
} 