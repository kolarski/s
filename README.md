<h1 align="center">🖥️ s</h1>
<h3 align="center">Terminal Sessions Made <i>Ridiculously</i> Simple</h3>
<p align="center">
  <a href="#installation"><img src="https://img.shields.io/badge/Install-Now-brightgreen" alt="Install Now"></a>
  <a href="https://github.com/kolarski/s/stargazers"><img src="https://img.shields.io/github/stars/kolarski/s" alt="Stars"></a>
  <a href="https://github.com/kolarski/s/blob/master/LICENSE"><img src="https://img.shields.io/github/license/kolarski/s" alt="License"></a>
</p>

<p align="center"><i>"Finally, a tool that makes screen sessions as simple as they should be!"</i></p>

## The Problem

If you've ever used the Linux `screen` command, you know the experience:

- Struggling to remember cryptic commands: `screen -list`, `screen -r 1234.pts-0.hostname`
- Hunting through messy session lists to find your work
- Accidentally creating multiple sessions when you meant to attach to existing ones
- Fighting with detached sessions and complicated syntax

**You're not alone.** Thousands of developers waste precious time on these frustrations every day.

## The Solution: `s`

`s` is a minimalist CLI tool that makes terminal session management **dead simple** while keeping all the power of `screen` that you actually need.

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

<p align="center">
  <img src="https://via.placeholder.com/600x300?text=Screenshot+of+s+in+action" alt="s in action" width="600">
</p>

## What Users Say

> "I've been using screen for 10 years and switching to `s` has been a game-changer. So much simpler!"

> "This tiny tool saves me at least 15 minutes of frustration every day."

> "I installed this on our entire team's machines. Everyone loves it."

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

## Contributing

We welcome all contributions! See our [CONTRIBUTING.md](CONTRIBUTING.md) guide to get started.

## License

GNU General Public License v3.0 - See [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/kolarski">Alex Kolarski</a>
</p>
