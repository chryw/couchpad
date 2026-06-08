# 🎮 Gamepad Mapper

A minimal CLI tool that maps game controller buttons to keyboard shortcuts. Built in Rust, works on macOS and Windows.

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
  --setup                Interactive setup wizard
  --init                 Create a new profile config file
  --edit                 Open config in your default editor
  --info                 Show controller info and keymap table
  --test                 Test mode: show button presses
  --list                 List and pick a profile to run
  --help, -h             Show full help
```

## Configuration

Profiles are JSON files stored in:
- **macOS:** `~/Library/Application Support/gamepad-mapper/<profile>.json`
- **Windows:** `%APPDATA%/gamepad-mapper/<profile>.json`

Example config:
```json
{
  "layer_button": "Home",
  "mappings": {
    "A": "return",
    "B": "escape",
    "DPadUp": "up",
    "DPadDown": "down",
    "LT": "super+shift+p",
    "Start": "super+s"
  },
  "layer_mappings": {
    "A": "super+s",
    "DPadUp": "alt+up",
    "Select": "ctrl+super+i"
  }
}
```

### Layer system

Hold the layer button (default: `Home`) + press another button for alternate mappings. This doubles your available actions.

### Cross-platform modifier

Use `super` in configs — it maps to `Cmd` on macOS and `Ctrl` on Windows. One profile works on both platforms.

## Button Names

| Button | Name |
|--------|------|
| Face buttons | `A`, `B`, `X`, `Y` |
| D-pad | `DPadUp`, `DPadDown`, `DPadLeft`, `DPadRight` |
| Bumpers | `LB`, `RB` |
| Triggers | `LT`, `RT` |
| Thumbsticks | `LS`, `RS` |
| Menu | `Select`, `Start` |
| Home | `Home` |

## Permissions

**macOS:** Grant Accessibility permission to your terminal app:
System Settings → Privacy & Security → Accessibility

## Example Profiles

See the [`profiles/`](profiles/) directory for ready-to-use configs:
- `default.json` — Basic mapping
- `vscode.json` — VS Code vibe coding with Copilot

## License

MIT
