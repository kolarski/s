use s::application::use_cases::list_screen_sessions::ListScreenSessions;
use s::application::use_cases::handle_screen_command::HandleScreenCommand;
use s::application::use_cases::kill_screen_session::KillScreenSession;
use s::infrastructure::adapters::screen_command::{ScreenCommand, ScreenCommandError};
use s::infrastructure::adapters::user_input::UserInput;
use s::infrastructure::adapters::system_checker::SystemChecker;
use s::infrastructure::adapters::system_error::SystemError;
use s::infrastructure::repositories::screen_repository::ScreenRepositoryImpl;
use s::presentation::formatters::table_formatter::TableFormatter;
use s::presentation::settings::{AppSettings, AppIcons};
use std::env;
use std::process;

// Version constant derived from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Parse command-line flags
    let args: Vec<String> = env::args().collect();
    let use_emoticons = args.iter().any(|arg| arg == "--emoticons" || arg == "-e");
    let show_version = args.iter().any(|arg| arg == "--version" || arg == "-v");
    
    // Show version and exit if requested
    if show_version {
        println!("s version {}", VERSION);
        process::exit(0);
    }
    
    // Create application settings
    let settings = AppSettings::new().with_emoticons(use_emoticons);
    
    // Check system compatibility
    let system_checker = SystemChecker::new();
    
    // Check if OS is compatible (Linux or macOS)
    if !system_checker.is_compatible_os() {
        let os = format!("{:?}", system_checker.detect_os());
        let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
        eprintln!("{}Error: {}", error_icon, SystemError::UnsupportedOs(os));
        process::exit(1);
    }
    
    // Check if screen is installed
    if !system_checker.is_screen_installed() {
        let instructions = system_checker.get_installation_instructions();
        let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
        eprintln!("{}Error: {}", error_icon, SystemError::ScreenNotInstalled(instructions));
        process::exit(1);
    }
    
    // Create dependencies
    let screen_command = ScreenCommand::new();
    let screen_repository = ScreenRepositoryImpl::new(screen_command);
    let user_input = UserInput::new();
    
    // Filter out the flags from the arguments
    let filtered_args: Vec<String> = args.into_iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect();
    
    // Handle the command based on the number of arguments (excluding flags)
    match filtered_args.len() {
        // If just "s" was entered, show the list of sessions
        1 => {
            // Create use case for listing sessions
            let list_sessions_use_case = ListScreenSessions::with_settings(screen_repository, settings);
            
            // Execute the use case
            match list_sessions_use_case.execute() {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        let info_icon = if settings.use_emoticons { AppIcons::info() } else { "" };
                        let screen_icon = if settings.use_emoticons { AppIcons::screen() } else { "" };
                        println!("{}No active screen sessions.", info_icon);
                        println!("{}To create a new session, run: s <session-name>", screen_icon);
                    } else {
                        // Format and display the results
                        let formatter = TableFormatter::with_settings(settings);
                        let formatted_output = formatter.format(&sessions);
                        println!("{}", formatted_output);
                    }
                }
                Err(error) => {
                    // Check for the special 'no sessions' error
                    if let Some(screen_err) = error.downcast_ref::<ScreenCommandError>() {
                        match screen_err {
                            ScreenCommandError::NoSessionsAvailable => {
                                let info_icon = if settings.use_emoticons { AppIcons::info() } else { "" };
                                let screen_icon = if settings.use_emoticons { AppIcons::screen() } else { "" };
                                println!("{}No active screen sessions.", info_icon);
                                println!("{}To create a new session, run: s <session-name>", screen_icon);
                            },
                            _ => {
                                let err_string = format!("{}", error);
                                if err_string.trim() == "Error executing screen command:" || 
                                   err_string.trim().is_empty() {
                                    // Empty error message usually means no sessions
                                    let info_icon = if settings.use_emoticons { AppIcons::info() } else { "" };
                                    let screen_icon = if settings.use_emoticons { AppIcons::screen() } else { "" };
                                    println!("{}No active screen sessions.", info_icon);
                                    println!("{}To create a new session, run: s <session-name>", screen_icon);
                                } else {
                                    let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
                                    eprintln!("{}Error: {}", error_icon, error);
                                }
                            }
                        }
                    } else {
                        let err_string = format!("{}", error);
                        if err_string.trim() == "Error executing screen command:" || 
                           err_string.trim().is_empty() {
                            // Empty error message usually means no sessions
                            let info_icon = if settings.use_emoticons { AppIcons::info() } else { "" };
                            let screen_icon = if settings.use_emoticons { AppIcons::screen() } else { "" };
                            println!("{}No active screen sessions.", info_icon);
                            println!("{}To create a new session, run: s <session-name>", screen_icon);
                        } else {
                            let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
                            eprintln!("{}Error: {}", error_icon, error);
                        }
                    }
                }
            }
        },
        
        // If "s example" or "s kill example" was entered
        2 => {
            let session_name = &filtered_args[1];
            
            // Create use case for handling screen command
            let handle_screen_command = HandleScreenCommand::with_settings(screen_repository, user_input, settings);
            
            // Execute the use case
            if let Err(error) = handle_screen_command.execute(session_name) {
                let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
                eprintln!("{}Error: {}", error_icon, error);
            }
        },
        
        // For commands with two arguments like "s kill example"
        3 => {
            let command = &filtered_args[1];
            let session_name = &filtered_args[2];
            
            match command.as_str() {
                "kill" => {
                    // Create use case for killing a session
                    let kill_session_use_case = KillScreenSession::with_settings(screen_repository, settings);
                    
                    // Execute the use case
                    if let Err(error) = kill_session_use_case.execute(session_name) {
                        let error_icon = if settings.use_emoticons { AppIcons::error() } else { "" };
                        eprintln!("{}Error: {}", error_icon, error);
                    }
                },
                _ => {
                    let warning_icon = if settings.use_emoticons { AppIcons::warning() } else { "" };
                    eprintln!("{}Unknown command: {}", warning_icon, command);
                    eprintln!("Available commands with two arguments:");
                    eprintln!("  s kill <name or ID>: Kill/remove a session");
                }
            }
        },
        
        // For more complex commands, we'll need to implement more handlers
        _ => {
            let info_icon = if settings.use_emoticons { AppIcons::info() } else { "" };
            eprintln!("{}Usage: s [options] [command] [identifier]", info_icon);
            eprintln!("Options:");
            eprintln!("  --emoticons, -e       : Enable emoticon icons in the output");
            eprintln!("Available commands:");
            eprintln!("  s                     : List all sessions");
            eprintln!("  s <name or ID>        : Connect to or create a session");
            eprintln!("  s kill <name or ID>   : Kill/remove a session");
        }
    }
}
