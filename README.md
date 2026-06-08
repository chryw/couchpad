# 🎮 Couchpad

Map a game controller to keyboard shortcuts. That's it.

No GUI, no hidden daemon, no account, no ads. A single binary you can read and understand. It runs in your terminal — you see exactly what it's doing, and it stops when you close it.

Works with any standard gamepad — Xbox, PlayStation, Switch, or cheap Bluetooth mini controllers. Useful for AI-assisted coding from the couch (Copilot Chat, Claude Code, Cursor) or any workflow where you want a few buttons to trigger shortcuts.

## Get started

1. Pair your controller via Bluetooth

2. Install — paste in your terminal:

   **macOS (Apple Silicon):**
   ```bash
   curl -fsSL https://github.com/YOUR_USERNAME/couchpad/releases/latest/download/couchpad-macos-arm64 -o /usr/local/bin/couchpad && chmod +x /usr/local/bin/couchpad
   ```

   **macOS (Intel):**
   ```bash
   curl -fsSL https://github.com/YOUR_USERNAME/couchpad/releases/latest/download/couchpad-macos-x86_64 -o /usr/local/bin/couchpad && chmod +x /usr/local/bin/couchpad
   ```

   **Windows (PowerShell):**
   ```powershell
   Invoke-WebRequest -Uri "https://github.com/YOUR_USERNAME/couchpad/releases/latest/download/couchpad-windows-x86_64.exe" -OutFile "$env:LOCALAPPDATA\couchpad.exe"
   ```

3. Run:

   ```bash
   couchpad
   ```

   First run walks you through setup — pick a built-in profile or map your own buttons interactively.

Keep the terminal open while you use it. Press Ctrl+C to stop.

> **macOS:** Grant Accessibility permission on first run:
> System Settings → Privacy & Security → Accessibility

## Why this exists

Most gamepad mappers are built for gaming — complex GUIs, dozens of options, background services, or restrictive licenses. If you just want to press a button and send a keyboard shortcut, they're overkill.

Couchpad is the tool you'd build yourself if you had a weekend. The entire codebase fits in your head.

## Learn more

- [Profile reference](docs/profile-reference.md) — format, button/key names, layers
- [VS Code guide](docs/vscode-guide.md) — couch coding with Copilot Chat + voice input

## Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/YOUR_USERNAME/couchpad.git
cd couchpad
cargo build --release
cargo install --path .
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for dev setup, architecture, and how to add features.

## License

MIT

