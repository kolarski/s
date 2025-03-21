/// Application settings for customizing behavior and output
#[derive(Clone, Copy)]
pub struct AppSettings {
    /// Whether to use emoticons/icons in the output
    pub use_emoticons: bool,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            use_emoticons: false,
        }
    }

    pub fn with_emoticons(mut self, use_emoticons: bool) -> Self {
        self.use_emoticons = use_emoticons;
        self
    }
}

/// Icons to use in the output when emoticons are enabled
pub struct AppIcons;

impl AppIcons {
    pub fn list() -> &'static str {
        "📋 "  // Clipboard
    }

    pub fn success() -> &'static str {
        "✅ "  // Check mark
    }

    pub fn error() -> &'static str {
        "❌ "  // X mark
    }

    pub fn info() -> &'static str {
        "ℹ️  "  // Information
    }
    
    pub fn warning() -> &'static str {
        "⚠️  "  // Warning
    }
    
    pub fn screen() -> &'static str {
        "🖥️  "  // Computer screen
    }
    
    pub fn create() -> &'static str {
        "🆕 "  // New
    }
    
    pub fn attach() -> &'static str {
        "🔗 "  // Link
    }
    
    pub fn detach() -> &'static str {
        "🔓 "  // Unlocked
    }
    
    pub fn kill() -> &'static str {
        "🗑️  "  // Trash
    }
    
    pub fn cancel() -> &'static str {
        "🚫 "  // No entry
    }
} 