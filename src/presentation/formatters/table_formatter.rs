use crate::domain::entities::screen_session::{ScreenSession, SessionStatus};
use crate::presentation::settings::{AppSettings, AppIcons};

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
        let list_icon = if self.settings.use_emoticons { AppIcons::list() } else { "" };
        let mut result = format!("{}Active Screen Sessions:\n", list_icon);
        result.push_str(&format!("{:<15} {:<30} {:<20} {:<10}\n", "ID", "NAME", "CREATED AT", "STATUS"));
        result.push_str(&"-".repeat(80));
        result.push('\n');

        // Add each session as a row
        for session in sessions {
            // Add status icon if emoticons are enabled
            let status_icon = if self.settings.use_emoticons {
                match session.status {
                    SessionStatus::Attached => AppIcons::attach(),
                    SessionStatus::Detached => AppIcons::detach(),
                    SessionStatus::Unknown => "",
                }
            } else {
                ""
            };
            
            result.push_str(&format!(
                "{:<15} {:<30} {:<20} {}{:<10}\n",
                session.id,
                session.name,
                session.created_at,
                status_icon,
                session.status.to_string()
            ));
        }

        result
    }
} 