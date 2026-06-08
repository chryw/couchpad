# Profiles

Pre-made profiles you can copy to your config directory and use immediately.

## Installation

Copy a profile to your config directory:

**macOS:**
```bash
cp profiles/vscode.json ~/Library/Application\ Support/gamepad-mapper/vscode.json
```

**Windows:**
```cmd
copy profiles\vscode.json %APPDATA%\gamepad-mapper\vscode.json
```

Then run with:
```bash
gamepad-mapper --profile vscode
```

## Available Profiles

### `default.json` — Generic Navigation

Designed for browsing, reading, and general app navigation. No app-specific shortcuts.

**Base layer:** Enter, Escape, Space, Backspace, Arrows, Prev/Next tab, Page Up/Down, Home/End

**Layer (Home+):** Paste, Copy, Ctrl+C, Select All, Tab switching, Back/Forward, Undo/Redo, Zoom, Find, Close tab

### `vscode.json` — VS Code Vibe Coding

Optimized for Copilot Chat-driven coding. The primary workflow is: open chat → speak/type prompt → accept suggestions → save.

**Base layer:** Enter, Escape, Tab (accept), Copilot Chat (LT), Quick Open (RT), Undo/Redo, Save

**Layer (Home+):** Close tab, Terminal, Interrupt, Command Palette, Run (F5), Find, Switch tabs

See the [VS Code Guide](../docs/vscode-guide.md) for the full workflow and voice input setup.

## Creating Your Own

```bash
# Start from an existing profile
cp profiles/default.json ~/Library/Application\ Support/gamepad-mapper/myprofile.json

# Edit it
gamepad-mapper --profile myprofile --edit

# Or use the interactive wizard
gamepad-mapper --profile myprofile --setup
```

See [Configuration docs](../docs/configuration.md) for button names, key names, and format reference.

