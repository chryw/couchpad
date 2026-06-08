# 🎮 Gamepad Mapper

A minimal CLI tool that maps game controller buttons to keyboard shortcuts. Built in Rust, works on macOS and Windows.

Works with any controller that supports standard gamepad input — Xbox, PlayStation, Switch Pro, and most Bluetooth mini controllers. Button labels in the UI auto-adjust for Xbox and Switch layouts (use `--layout` to override).

Perfect for couch coding with [Copilot Chat](docs/vscode-guide.md) + voice input.

## Install

### Download (recommended)

From [Releases](../../releases):

- **macOS (Apple Silicon):** `gamepad-mapper-macos-arm64`
- **macOS (Intel):** `gamepad-mapper-macos-x86_64`
- **Windows:** `gamepad-mapper-windows-x86_64.exe`

```bash
# macOS
chmod +x gamepad-mapper-macos-arm64
sudo mv gamepad-mapper-macos-arm64 /usr/local/bin/gamepad-mapper
```

On Windows, move the `.exe` to a folder in your `PATH`, or run it directly.

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/YOUR_USERNAME/gamepad-mapper.git
cd gamepad-mapper
cargo build --release
cargo install --path .
```

## Quick start

1. Pair your Bluetooth controller (System Settings → Bluetooth on macOS, Settings → Bluetooth on Windows)
2. Run:

```bash
# Use a built-in profile (works immediately, no setup needed)
gamepad-mapper --profile vscode

# Or create your own with the interactive wizard
gamepad-mapper --setup

# See all options
gamepad-mapper --help
```

> **macOS:** On first run, grant Accessibility permission to your terminal app:
> System Settings → Privacy & Security → Accessibility

## Documentation

| Doc | What's in it |
|-----|--------------|
| [Profile reference](docs/profile-reference.md) | Profile format, built-in profiles, button/key names, layers |
| [VS Code guide](docs/vscode-guide.md) | Vibe coding with Copilot Chat + voice input |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for dev setup, architecture, and how to add features.

## License

MIT

