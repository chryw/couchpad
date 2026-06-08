# Contributing

## Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/YOUR_USERNAME/gamepad-mapper.git
cd gamepad-mapper
cargo build
```

## Development

```bash
cargo build              # Build debug binary
cargo build --release    # Build optimized binary
cargo run -- --help      # Run with arguments
cargo run -- --test      # Test with a connected controller
```

## Project Structure

```
src/
  main.rs              CLI logic, arg parsing, event loop, setup wizard
  config.rs            Profile loading/saving, path resolution
  keymap.rs            Key name → keycode mappings, combo parsing
  emitter_macos.rs     macOS CGEvent key simulation
  emitter_windows.rs   Windows SendInput key simulation
docs/
  help/                Plain text fragments embedded in --help via include_str!()
  configuration.md     Config reference (web docs)
  vscode-guide.md      VS Code vibe coding guide (web docs)
profiles/              Example profile JSON files
```

## Adding a Key

1. Add the keycode to `src/keymap.rs` in `build_keycode_map()`
2. Add the Windows VK code to `src/emitter_windows.rs` in `build_vk_map()`
3. Update `docs/help/keys.md` with the new key name

## Adding a Profile

1. Create a new JSON file in `profiles/`
2. Add a description to `docs/profiles.md`
3. Optionally add a dedicated guide in `docs/`

## Guidelines

- No panics in production paths — use graceful error messages
- Config always uses Xbox button convention (A=bottom, B=right, X=left, Y=top)
- `super` modifier for cross-platform shortcuts
- Test with a real controller before submitting controller-related changes
