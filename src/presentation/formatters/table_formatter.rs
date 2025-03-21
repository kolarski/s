use crate::domain::entities::screen_session::ScreenSession;
use crate::presentation::settings::AppSettings;

/// Formatter for displaying screen sessions in a table format
pub struct TableFormatter {
    settings: AppSettings,
}

impl TableFormatter {
    pub fn new() -> Self {
        Self {
            settings: AppSettings::new(),
        }
    }
    
    pub fn with_settings(settings: AppSettings) -> Self {
        Self { settings }
    }

    /// Format a list of screen sessions as a table
    pub fn format(&self, sessions: &[ScreenSession]) -> String {
        if sessions.is_empty() {
            return "No active screen sessions.".to_string();
        }

        // Create header for the table
        let mut result = String::new();
        
        // Column widths can be adjusted based on settings if needed in the future
        let _column_settings = &self.settings; // Using settings reference to avoid the warning
        
        result.push_str(&format!("{:<15} {:<30} {:<20}\n", "ID", "NAME", "CREATED AT"));
        result.push_str(&"-".repeat(65));
        result.push('\n');

        // Add each session as a row
        for session in sessions {
            result.push_str(&format!(
                "{:<15} {:<30} {:<20}\n",
                session.id,
                session.name,
                session.created_at
            ));
        }

        result
    }
} 