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
  profile.rs           Profile loading/saving, path resolution
  keymap.rs            Key name → keycode mappings, combo parsing
  emitter_macos.rs     macOS CGEvent key simulation
  emitter_windows.rs   Windows SendInput key simulation
docs/
  help/                Plain text fragments embedded in --help via include_str!()
  configuration.md     Profile format and reference (web docs)
  vscode-guide.md      VS Code vibe coding guide (web docs)
profiles/              Built-in profile JSON files
```

## Adding a Key

1. Add the keycode to `src/keymap.rs` in the platform-specific `build_keycode_map_*()` functions
2. Update `docs/help/keys.md` with the new key name

## Adding a Profile

1. Create a new JSON file in `profiles/`
2. Add a description to the "Using Built-in Profiles" section in `docs/configuration.md`
3. Optionally add a dedicated guide in `docs/`

## Guidelines

- No panics in production paths — use graceful error messages
- Profiles always use Xbox button convention (A=bottom, B=right, X=left, Y=top)
- `super` modifier for cross-platform shortcuts
- Test with a real controller before submitting controller-related changes
