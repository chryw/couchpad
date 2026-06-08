# 🎮 Gamepad Mapper

A minimal command-line tool that maps game controller buttons to keyboard shortcuts. Runs in a terminal alongside your editor. Built in Rust, works on macOS and Windows.

Works with any controller that supports standard gamepad input — Xbox, PlayStation, Switch Pro, and most Bluetooth mini controllers. Button labels in the UI auto-adjust for Xbox and Switch layouts (use `--layout` to override).

Perfect for couch coding with [Copilot Chat](docs/vscode-guide.md) + voice input.

## Get started

1. Pair your Bluetooth controller (System Settings → Bluetooth on macOS, Settings → Bluetooth on Windows)

2. Download the binary for your platform from [Releases](../../releases) and open a terminal:

   **macOS:**
   ```bash
   chmod +x gamepad-mapper-macos-arm64
   sudo mv gamepad-mapper-macos-arm64 /usr/local/bin/gamepad-mapper
   ```

   **Windows (PowerShell):**
   ```powershell
   move gamepad-mapper-windows-x86_64.exe C:\Users\YOU\bin\gamepad-mapper.exe
   ```

3. Run it:

   ```bash
   # Interactive wizard — maps buttons to your choice of shortcuts
   gamepad-mapper --setup

   # Or start immediately with a pre-made VS Code profile
   gamepad-mapper --profile vscode
   ```

Keep the terminal open — the tool runs in the foreground and maps buttons until you press Ctrl+C.

> **macOS:** On first run, grant Accessibility permission to your terminal app:
> System Settings → Privacy & Security → Accessibility

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/YOUR_USERNAME/gamepad-mapper.git
cd gamepad-mapper
cargo build --release
cargo install --path .
```

## Learn more

- [Profile reference](docs/profile-reference.md) — format, built-in profiles, button/key names, layers
- [VS Code guide](docs/vscode-guide.md) — vibe coding with Copilot Chat + voice input

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for dev setup, architecture, and how to add features.

## License

MIT

