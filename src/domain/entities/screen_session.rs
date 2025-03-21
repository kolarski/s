/// Represents the status of a screen session
#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    Attached,
    Detached,
    Unknown,
}

impl SessionStatus {
    pub fn from_str(status: &str) -> Self {
        match status {
            "Attached" => SessionStatus::Attached,
            "Detached" => SessionStatus::Detached,
            _ => SessionStatus::Unknown,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SessionStatus::Attached => "Attached".to_string(),
            SessionStatus::Detached => "Detached".to_string(),
            SessionStatus::Unknown => "Unknown".to_string(),
        }
    }
}

/// Represents a screen session in the system
#[derive(Debug, Clone)]
pub struct ScreenSession {
    /// Unique identifier for the session
    pub id: String,
    /// Name of the session (typically includes terminal info)
    pub name: String,
    /// When the session was created
    pub created_at: String,
    /// Current status of the session
    pub status: SessionStatus,
}

impl ScreenSession {
    pub fn new(id: String, name: String, created_at: String, status: SessionStatus) -> Self {
        Self {
            id,
            name,
            created_at,
            status,
        }
    }
} 