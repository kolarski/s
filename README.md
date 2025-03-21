# s - A User-Friendly Wrapper for Linux Screen

`s` is a lightweight, user-friendly command-line utility that wraps the Linux `screen` command, providing a more intuitive and developer-friendly experience.

## Why s?

The standard Linux `screen` command is incredibly powerful but can be difficult to use due to its:

- Complex flag combinations
- Non-intuitive syntax
- Hard-to-remember commands

With `s`, you can harness the power of `screen` without the cognitive overhead of remembering all those complicated commands and flags.

## Features

- Simple, intuitive commands
- Human-readable syntax
- All the power of `screen` with none of the complexity
- Sensible defaults for common operations

## Installation

### Quick Install (Recommended)

Install or upgrade to the latest version using our installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/s/main/install.sh | bash
```

This script will:

- Detect your operating system
- Download the appropriate binary for your system
- Install it to a suitable location (typically /usr/local/bin)
- Add the installation directory to your PATH if needed
- Display the installed version information

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/s.git
cd s

# Build and install
cargo install --path .
```

## Usage

### Basic Commands

```bash
# List all sessions
s

# Create and attach to a session
s <session-name>

# Kill/remove a session
s kill <session-name>
```

### Options

```bash
# Enable emoticon icons in the output
s --emoticons
# or
s -e
```

## How It Works / Documentation

`s` is designed to simplify your workflow with screen sessions through a minimal set of commands:

### Listing Sessions

Simply running `s` without any arguments will list all your active screen sessions in a clean table format:

```
ID              NAME                           CREATED AT
-----------------------------------------------------------------
1200543         test                           21.03.2025 11:51:37
```

### Creating and Attaching to Sessions

To create a new session or attach to an existing one:

```bash
s <session-name>
```

If the session doesn't exist, `s` will ask if you want to create it:

```
Screen 'my-session' does not exist. Create and attach? [Y/n]
```

If the session exists but is detached, `s` will automatically attach to it.

If the session exists and is already attached elsewhere, `s` will ask if you want to detach the other connection and reattach:

```
Screen 'my-session' is already attached. Detach and reattach? [Y/n]
```

### Killing/Removing Sessions

To remove a screen session:

```bash
s kill <session-name>
```

This will terminate the specified session.

## Dependencies

- Rust (for building from source)
- Linux `screen` command (must be installed on your system)
- Compatible with Linux and macOS

## Roadmap

- [ ] Configuration file support
- [ ] Custom keyboard shortcuts
- [ ] Session templates
- [ ] Integration with tmux as an alternative backend

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
