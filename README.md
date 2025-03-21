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
# Start a new session
s new [session-name]

# List all sessions
s list

# Attach to a session
s attach [session-name]

# Detach from current session
s detach

# Kill a session
s kill [session-name]
```

### Examples

```bash
# Start a new session named "dev"
s new dev

# Start a new session with a command
s new web "npm start"

# List all running sessions
s list

# Attach to existing session
s attach dev

# Split the current window horizontally
s split

# Split the current window vertically
s vsplit

# Navigate between panes
s next
s prev

# Create a new window
s window
```

## How It Works

`s` translates its user-friendly commands into the appropriate `screen` commands behind the scenes. It's designed to provide all the functionality of `screen` without requiring you to memorize complex command sequences.

## Dependencies

- Rust (for building from source)
- Linux `screen` command (must be installed on your system)

## Roadmap

- [ ] Configuration file support
- [ ] Custom keyboard shortcuts
- [ ] Session templates
- [ ] Integration with tmux as an alternative backend

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
