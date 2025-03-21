<h1 align="center">🖥️ s</h1>
<h3 align="center">Terminal Sessions Made <i>Ridiculously</i> Simple</h3>
<p align="center">
  <a href="#installation"><img src="https://img.shields.io/badge/Install-Now-brightgreen" alt="Install Now"></a>
  <a href="https://github.com/kolarski/s/stargazers"><img src="https://img.shields.io/github/stars/kolarski/s" alt="Stars"></a>
  <a href="https://github.com/kolarski/s/blob/master/LICENSE"><img src="https://img.shields.io/github/license/kolarski/s" alt="License"></a>
  <a href="https://kolarski.github.io/s/"><img src="https://img.shields.io/badge/Website-Visit%20Now-blue" alt="Website"></a>
</p>

<p align="center"><i>"Finally, a tool that makes screen sessions as simple as they should be!"</i></p>

<div align="center">
  <strong>🔰 New to terminal sessions?</strong> <a href="#terminal-sessions">Click here to learn the basics first →</a>
</div>

<p align="center">
  <img src="/assets/image.png" alt="s in action - simple terminal session management" width="600">
</p>

## Meet `s`

`s` is a minimalist wrapper around `screen` that makes terminal session management **dead simple** while keeping all the power of the original tool that you actually need.

### ⏱️ Try It in 10 Seconds

```bash
# 1. Install
curl -fsSL https://raw.githubusercontent.com/kolarski/s/master/install.sh | bash

# 2. List sessions
s

# 3. Create/attach to a session
s my-project

# 4. Detach when done (standard screen command)
Ctrl+A, then D

# 5. Kill a session when completely finished
s kill my-project
```

#### What to expect:

- When you run `s` you'll see a clean table of all sessions
- `s my-project` smartly attaches to existing sessions or confirms before creating new ones
- Standard screen shortcuts work inside sessions

## Why `s` Exists

The Linux `screen` utility is an incredibly powerful and versatile tool with a rich feature set developed over decades. While indispensable for server management and remote work, its depth comes with complexity that can impact everyday workflows:

- Commands and flags that are challenging to remember when you just need to quickly manage sessions
- Multi-step processes required for common tasks like reattaching to already-attached sessions
- Feature-rich interface where many advanced capabilities are rarely used but add complexity
- Session naming and management that can become unwieldy as you accumulate sessions
- Command syntax that requires precise recall when you're focused on your actual work

`s` was created out of respect for `screen`'s capabilities while acknowledging that developers often need a streamlined interface for their most common terminal session tasks. It preserves all the power of `screen` but provides a simplified command layer for day-to-day usage.

## Core Commands You'll Use Daily

| Task                       | Command       | What it Does                                     |
| -------------------------- | ------------- | ------------------------------------------------ |
| List all sessions          | `s`           | Shows a clean, human-readable table              |
| Create/attach to a session | `s name`      | Creates if it doesn't exist, attaches if it does |
| Kill a session             | `s kill name` | Removes the session                              |

## Why Developers Stick With `s`

- **One-Letter Simplicity**: Just type `s` and you're productive
- **Zero Learning Curve**: If you can type, you can use it
- **Smart Context Awareness**: Creates or attaches based on what you need
- **Increased Productivity**: Save ~5-10 seconds on every session interaction (adds up to hours saved!)
- **Designed for Real Workflows**: Built by developers for actual day-to-day usage patterns

## What Users Say

> "I've been using screen for 10 years and switching to `s` has been a game-changer. So much simpler!"

> "This tiny tool saves me at least 15 minutes of frustration every day."

> "I installed this on our entire team's machines. Everyone loves it."

<a name="terminal-sessions"></a>

## Terminal Sessions Explained

### What are Terminal Sessions?

Terminal sessions (via tools like `screen` or `tmux`) let you:

- **Keep processes running** even after you disconnect from SSH or close your terminal
- **Resume your work** exactly where you left off
- **Run multiple terminal windows** within a single connection
- **Share terminal sessions** with teammates for collaboration

If you're using SSH to connect to servers or running long processes that you don't want to lose when your connection drops, terminal sessions are essential.

### New to Terminal Sessions?

If you've never used terminal sessions before, here's what you can do once you're inside a session:

- **Run commands that continue even if you disconnect** - Start long processes and they'll keep running after you close your terminal
- **Create multiple terminal windows** (Ctrl+A, C) - Work in different contexts without opening new terminals
- **Switch between windows** (Ctrl+A, 0-9) - Toggle between your windows like tabs
- **Disconnect and reconnect later** exactly where you left off (Ctrl+A, D to detach)
- **Get help anytime** - Press Ctrl+A, ? for the complete help menu

`s` makes all of this ridiculously simple by providing an intuitive interface to these powerful features.

## <a name="installation"></a>Installation

### ✅ Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/kolarski/s/master/install.sh | bash
```

The script intelligently:

- Detects your OS
- Downloads the appropriate binary
- Installs to the correct location
- Adds to your PATH if needed

### Manual Binary Download

You can also download the pre-compiled binary directly from the [GitHub releases page](https://github.com/kolarski/s/releases):

1. Download the appropriate binary for your system (Linux or macOS)
2. Make it executable: `chmod +x s-linux-amd64` or `chmod +x s-macos-amd64`
3. Move it to a directory in your PATH: `mv s-linux-amd64 /usr/local/bin/s` or `mv s-macos-amd64 /usr/local/bin/s`

### From Source

```bash
git clone https://github.com/kolarski/s.git
cd s
cargo install --path .
```

## Requirements

- Linux `screen` command must be installed
- Compatible with Linux and macOS

## Under the Hood

`s` is built with modern software engineering practices that ensure reliability, maintainability, and performance:

- **📦 Written in [Rust](https://www.rust-lang.org/)**: Delivering memory safety without a garbage collector, thread safety without data races, and abstraction without overhead
- **🏛️ [Domain Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)**: Focused on core domain logic and complex domain models to solve real developer problems
- **🧱 [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)**: Separation of concerns that makes the codebase modular, testable, and easy to maintain
- **🚀 Zero Dependencies**: Completely self-contained with no external libraries, making it lightweight, fast, and secure

`s` is intentionally designed as a thin wrapper around the robust `screen` utility rather than reimplementing its functionality. This approach leverages the stability and feature set of `screen` while providing a dramatically simplified interface for everyday use.

For developers interested in contributing or building similar tools, our codebase serves as an excellent example of applying these principles to a focused utility.

## Help Us Grow

If `s` has made your terminal life better:

1. **⭐ Star the repo** to help others discover it
2. **🐦 Share on social** with #TerminalProductivity and tag us
3. **👩‍💻 Introduce it** at your next team meeting
4. **🍴 Contribute** features or fixes you'd like to see
5. **🌐 Visit our [website](https://kolarski.github.io/s/)** to learn more about the project

## Contributing

We welcome all contributions! See our [CONTRIBUTING.md](CONTRIBUTING.md) guide to get started.

## License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/kolarski">Alex Kolarski</a>
</p>
