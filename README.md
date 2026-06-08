# 🎮 Gamepad Mapper

A minimal CLI tool that maps game controller buttons to keyboard shortcuts. Built in Rust, works on macOS and Windows.

Perfect for couch coding with [Copilot Chat](docs/vscode-guide.md) + voice input.

## Install

### Download (recommended)

Grab the latest binary from [Releases](../../releases):

- **macOS (Apple Silicon):** `gamepad-mapper-macos-arm64`
- **macOS (Intel):** `gamepad-mapper-macos-x86_64`
- **Windows:** `gamepad-mapper-windows-x86_64.exe`

After downloading:

```bash
# macOS
chmod +x gamepad-mapper-macos-arm64
sudo mv gamepad-mapper-macos-arm64 /usr/local/bin/gamepad-mapper
```

On Windows, move the `.exe` to a folder in your `PATH`, or run it directly.

### Build from source

```bash
git clone https://github.com/YOUR_USERNAME/gamepad-mapper.git
cd gamepad-mapper
cargo build --release
# Binary at: target/release/gamepad-mapper(.exe on Windows)
```

## Quick start

```bash
# 1. Create a default profile
gamepad-mapper --init

# 2. Interactive setup (pick actions, press buttons)
gamepad-mapper --setup

# 3. Run
gamepad-mapper
```

Or use a built-in profile:

```bash
# macOS
cp profiles/vscode.json ~/Library/Application\ Support/gamepad-mapper/vscode.json

# Windows
copy profiles\vscode.json %APPDATA%\gamepad-mapper\vscode.json
```

```bash
gamepad-mapper --profile vscode
```

## Usage

```
gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start with the default profile
  --profile <name>       Use a named profile
  --layout <type>        Override controller layout (xbox or switch)
  --setup                Interactive setup wizard
  --init                 Create a new profile
  --edit                 Open profile in your default editor
  --validate             Check profile for errors
  --info                 Show controller info and current mappings
  --test                 Test mode: show button presses
  --list                 List and pick a profile to run
  --load <path>          Load a profile from a specific file path
  --help, -h             Show full help
```

## Documentation

| Doc | What's in it |
|-----|--------------|
| [Profile reference](docs/profile-reference.md) | Profile format, built-in profiles, button/key names, layers |
| [VS Code guide](docs/vscode-guide.md) | Vibe coding with Copilot Chat + voice input |

## Permissions

**macOS:** Grant Accessibility permission to your terminal app:
System Settings → Privacy & Security → Accessibility

**Windows:** No extra permissions needed. Run from any terminal.

## License

MIT

