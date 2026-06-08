# Example Profiles

Pre-made profiles you can copy to your config directory.

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

### `default.json`
Basic gamepad-to-keyboard mapping. No layer mappings. Good starting point.

### `vscode.json`
Optimized for VS Code vibe coding with Copilot. Includes layer mappings
(hold Home + button) for extended actions like switching tabs, moving lines,
toggling Copilot Chat, and more.
