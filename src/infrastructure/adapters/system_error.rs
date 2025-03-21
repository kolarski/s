use std::error::Error;
use std::fmt;

/// SystemError represents errors related to system compatibility
#[derive(Debug)]
pub enum SystemError {
    /// Error when the operating system is not supported
    UnsupportedOs(String),
    
    /// Error when the screen command is not installed
    ScreenNotInstalled(String),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::UnsupportedOs(os) => {
                write!(f, "Unsupported operating system: {}. This tool only supports Linux and macOS.", os)
            },
            SystemError::ScreenNotInstalled(instructions) => {
                write!(f, "The 'screen' command is not installed on your system.\n{}", instructions)
            },
        }
    }
}

impl Error for SystemError {} 