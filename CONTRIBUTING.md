# Contributing

## Key concepts

A few terms in the codebase come from third-party libraries and platform APIs:

- **South, East, North, West** — how [gilrs](https://docs.rs/gilrs/latest/gilrs/ev/enum.Button.html) names the four face buttons by position, not label. South=bottom (Xbox A), East=right (Xbox B), North=top (Xbox Y), West=left (Xbox X).
- **`super` modifier** — a cross-platform abstraction we define. It resolves to `Cmd` (⌘) on macOS and `Ctrl` on Windows, so one profile works on both platforms.
- **CGEvent / CGKeyCode** — macOS [Core Graphics](https://developer.apple.com/documentation/coregraphics/cgevent) API for synthesizing keyboard input. Keycodes are hardware scan codes, not characters.
- **SendInput / VK codes** — Windows [SendInput](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput) API for simulating input. Uses [Virtual-Key codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).
- **gilrs** — [Game Input Library for Rust](https://docs.rs/gilrs/latest/gilrs/). Provides cross-platform gamepad detection and event polling.
- **Layers** — inspired by keyboard firmware like [QMK](https://docs.qmk.fm/feature_layers). Holding a modifier button (default: `Home`) switches to an alternate set of mappings, doubling the number of available actions without extra buttons. Same concept as Shift on a keyboard giving you uppercase letters.

## Setup

**Prerequisites:**
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- A Bluetooth or USB game controller (for testing)
- **macOS:** Accessibility permission granted to your terminal (System Settings → Privacy & Security → Accessibility)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/YOUR_USERNAME/couchpad.git
cd couchpad
cargo build
```

## Development

```bash
cargo build              # Build debug binary
cargo build --release    # Build optimized binary
cargo run -- --help      # Run with arguments
cargo run -- --test      # Test with a connected controller
```

## Project structure

```
src/
  main.rs              CLI logic, arg parsing, event loop, setup wizard
  profile.rs           Profile loading/saving, path resolution
  keymap.rs            Key name → keycode mappings, combo parsing
  emitter_macos.rs     macOS CGEvent key simulation
  emitter_windows.rs   Windows SendInput key simulation
docs/
  help/                Plain text fragments embedded in --help via include_str!()
  profile-reference.md Profile format, button/key names, layers (web docs)
  vscode-guide.md      VS Code vibe coding guide (web docs)
profiles/              Built-in profile JSON files
```

## Adding a key

1. Add the keycode to `src/keymap.rs` in the platform-specific `build_keycode_map_*()` functions
2. Update `docs/help/keys.md` with the new key name

## Adding a profile

1. Create a new JSON file in `profiles/`
2. Register it in `src/profile.rs` in the `builtin_profile()` and `builtin_profile_names()` functions
3. Add a description to the "Built-in profiles" section in `docs/profile-reference.md`
4. Optionally add a dedicated guide in `docs/`

## Guidelines

- No panics in production paths — use graceful error messages
- Profiles always use Xbox button convention (A=bottom, B=right, X=left, Y=top)
- `super` modifier for cross-platform shortcuts
- Test with a real controller before submitting controller-related changes
