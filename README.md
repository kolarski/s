<p align="center">
  <span style="font-size: 100px;">🖥️</span><br>
  <span style="font-size: 50px;">⚡ <b>s</b> ⚡</span>
</p>

# s - Terminal Sessions Made Simple

`s` is a lightweight CLI tool that makes Linux `screen` sessions delightfully easy to use for everyday development tasks.

## Try It Now (Takes 10 Seconds)

```bash
# Install it
curl -fsSL https://raw.githubusercontent.com/kolarski/s/refs/heads/master/install.sh | bash

# List all sessions
s

# Create/attach to a session
s my-project

# That's it! You're already using s!
```

## What You Just Experienced

You've just used `s` to manage your terminal sessions with simple, intuitive commands. No more complex flags, confusing syntax, or hard-to-remember commands from the standard `screen` utility.

## Why Developers Love `s`

The standard Linux `screen` command is incredibly powerful and feature-rich—arguably one of the most versatile terminal utilities available. However, for most day-to-day software development tasks, it's significantly overpowered.

Most developers need a handful of simple operations:

- List sessions
- Create/attach to a session
- Kill a session

`s` focuses exclusively on these common developer workflows, stripping away the complexity while maintaining the core functionality most developers actually use:

- **Simple Commands**: Replace cryptic screen commands with intuitive ones
- **Human-Friendly**: Clear prompts and clean output format
- **Zero Learning Curve**: If you can type `s`, you can use it effectively
- **Smart Defaults**: It just does what you expect, aligned with typical development patterns

## How It Makes Your Life Better

### Seamless Session Management

Simply type `s` to view all your sessions in a clean table:

```
ID              NAME                           CREATED AT
-----------------------------------------------------------------
1200543         test                           21.03.2025 11:51:37
```

### Intuitive Session Creation and Attachment

```bash
s <session-name>
```

`s` is smart enough to:

- Create a new session if it doesn't exist (with confirmation)
- Attach to existing detached sessions automatically
- Prompt to detach/reattach if the session is already attached elsewhere

### Effortless Session Cleanup

```bash
s kill <session-name>
```

## More Advanced Features

```bash
# Enable emoticon icons in the output
s --emoticons
# or
s -e
```

## Installation Options

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/kolarski/s/refs/heads/master/install.sh | bash
```

The script handles everything for you:

- Detects your OS
- Downloads the right binary
- Installs to the right location
- Updates your PATH if needed

### From Source

```bash
# Clone the repository
git clone https://github.com/kolarski/s.git
cd s

# Build and install
cargo install --path .
```

## Dependencies

- Linux `screen` command must be installed
- Compatible with Linux and macOS

## Share the Joy

Love using `s`? Here's how to spread the word:

- Star the GitHub repo
- Share on social media with #sMadeSimple
- Tell your team about it in your next standup

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.
