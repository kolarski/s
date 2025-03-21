use std::process::Command;

#[derive(Debug)]
pub enum OperatingSystem {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

pub struct SystemChecker;

impl SystemChecker {
    pub fn new() -> Self {
        SystemChecker
    }

    pub fn detect_os(&self) -> OperatingSystem {
        if cfg!(target_os = "linux") {
            OperatingSystem::Linux
        } else if cfg!(target_os = "macos") {
            OperatingSystem::MacOS
        } else if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else {
            OperatingSystem::Unknown
        }
    }

    pub fn is_compatible_os(&self) -> bool {
        matches!(self.detect_os(), OperatingSystem::Linux | OperatingSystem::MacOS)
    }

    pub fn is_screen_installed(&self) -> bool {
        let output = Command::new("which")
            .arg("screen")
            .output();
        
        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub fn get_installation_instructions(&self) -> String {
        match self.detect_os() {
            OperatingSystem::Linux => {
                // Try to detect Linux distribution
                if let Ok(output) = Command::new("cat").arg("/etc/os-release").output() {
                    let os_release = String::from_utf8_lossy(&output.stdout);
                    
                    if os_release.contains("ID=ubuntu") || os_release.contains("ID=debian") {
                        return "To install screen on Ubuntu/Debian: sudo apt-get install screen".to_string();
                    } else if os_release.contains("ID=fedora") || os_release.contains("ID=centos") || os_release.contains("ID=rhel") {
                        return "To install screen on Fedora/CentOS/RHEL: sudo dnf install screen".to_string();
                    } else if os_release.contains("ID=arch") {
                        return "To install screen on Arch Linux: sudo pacman -S screen".to_string();
                    }
                }
                
                // Fallback for Linux if we can't detect the specific distribution
                "To install screen on Linux, use your distribution's package manager (apt, dnf, pacman, etc.)".to_string()
            },
            OperatingSystem::MacOS => {
                "To install screen on macOS: brew install screen".to_string()
            },
            _ => "The 'screen' command is required but your operating system is not supported.".to_string(),
        }
    }
} 