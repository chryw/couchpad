# 🎮 Gamepad Mapper

A minimal CLI tool that maps game controller buttons to keyboard shortcuts. Built in Rust, works on macOS and Windows.

Perfect for couch coding with [Copilot Chat](docs/vscode-guide.md) + voice input.

## Install

### Download (recommended)

Grab the latest binary from [Releases](../../releases):

- **macOS (Apple Silicon):** `gamepad-mapper-macos-arm64`
- **macOS (Intel):** `gamepad-mapper-macos-x86_64`
- **Windows:** `gamepad-mapper-windows-x86_64.exe`

```bash
# macOS: download, make executable, move to PATH
chmod +x gamepad-mapper-macos-arm64
sudo mv gamepad-mapper-macos-arm64 /usr/local/bin/gamepad-mapper
```

### Build from source

```bash
git clone https://github.com/YOUR_USERNAME/gamepad-mapper.git
cd gamepad-mapper
cargo build --release
# Binary at: target/release/gamepad-mapper
```

## Quick Start

```bash
# 1. Create a default config
gamepad-mapper --init

# 2. Interactive setup (pick actions, press buttons)
gamepad-mapper --setup

# 3. Or just run with the default profile
gamepad-mapper
```

## Usage

```
gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start the mapper with the default profile
  --profile <name>       Use a specific profile
  --layout <type>        Override controller layout (xbox or switch)
  --setup                Interactive setup wizard
  --init                 Create a new profile config file
  --edit                 Open config in your default editor
  --info                 Show controller info and keymap table
  --test                 Test mode: show button presses
  --list                 List and pick a profile to run
  --config <path>        Use a custom config file path
  --help, -h             Show full help
```

## Profiles

Ready-to-use profiles in [`profiles/`](profiles/):

| Profile | Use Case |
|---------|----------|
| [`default.json`](profiles/default.json) | Generic navigation — browse, scroll, tabs, clipboard |
| [`vscode.json`](profiles/vscode.json) | VS Code vibe coding with Copilot Chat |

Copy to your config directory and run:
```bash
cp profiles/vscode.json ~/Library/Application\ Support/gamepad-mapper/vscode.json
gamepad-mapper --profile vscode
```

## Documentation

| Doc | Description |
|-----|-------------|
| [Configuration](docs/configuration.md) | Config format, button names, key names, layers, cross-platform |
| [VS Code Guide](docs/vscode-guide.md) | Vibe coding with Copilot Chat + voice input |
| [Profiles](docs/profiles.md) | Profile installation and descriptions |

## Permissions

**macOS:** Grant Accessibility permission to your terminal app:
System Settings → Privacy & Security → Accessibility

## License

MIT

