# Configuration

## Profile Location

Profiles are JSON files stored in:
- **macOS:** `~/Library/Application Support/gamepad-mapper/<name>.json`
- **Windows:** `%APPDATA%/gamepad-mapper/<name>.json`

Use `--profile <name>` to select a profile. Omit it to use `default.json`.

## Getting Started

```bash
# Create a default profile
gamepad-mapper --init

# Interactive wizard — pick actions, press buttons to map them
gamepad-mapper --setup

# Or create a named profile
gamepad-mapper --profile vscode --init
```

## Using Built-in Profiles

Ready-to-use profiles ship in the [`profiles/`](../profiles/) directory:

| Profile | Use Case |
|---------|----------|
| `default.json` | Generic navigation — browse, scroll, tabs, clipboard |
| `vscode.json` | VS Code vibe coding with Copilot Chat ([guide](vscode-guide.md)) |

Install one by copying it to your profile directory:

```bash
# macOS
cp profiles/vscode.json ~/Library/Application\ Support/gamepad-mapper/vscode.json

# Windows
copy profiles\vscode.json %APPDATA%\gamepad-mapper\vscode.json
```

Then run:
```bash
gamepad-mapper --profile vscode
```

### `default.json` — Generic Navigation

Designed for browsing, reading, and general app navigation. No app-specific shortcuts.

- **Base layer:** Enter, Escape, Space, Backspace, Arrows, Prev/Next tab, Page Up/Down, Home/End
- **Layer (Home+):** Paste, Copy, Ctrl+C, Select All, Tab switching, Back/Forward, Undo/Redo, Zoom, Find, Close tab

### `vscode.json` — VS Code Vibe Coding

Optimized for Copilot Chat-driven coding. The workflow: open chat → speak/type prompt → accept suggestions → save.

- **Base layer:** Enter, Escape, Tab (accept), Copilot Chat (LT), Quick Open (RT), Undo/Redo, Save
- **Layer (Home+):** Close tab, Terminal, Interrupt, Command Palette, Run (F5), Find, Switch tabs

## Profile Format

```json
{
  "layer_button": "Home",
  "mappings": {
    "A": "return",
    "B": "escape",
    "DPadUp": "up",
    "LT": "super+shift+p"
  },
  "layer_mappings": {
    "A": "super+s",
    "DPadUp": "alt+up"
  }
}
```

| Field | Description |
|-------|-------------|
| `layer_button` | Button that activates layer 2 when held (default: `"Home"`) |
| `mappings` | Button → key combo mappings for the base layer |
| `layer_mappings` | Button → key combo mappings when layer button is held |

## Layer System

Hold the layer button (default: `Home`) + press another button for alternate mappings. This doubles your available actions without needing more buttons.

Example: `Home + A` sends `super+s` (Save) while `A` alone sends `return`.

## Button Names

Profiles always use Xbox-convention names regardless of which controller is connected:

| Position | Name | Xbox Label | Switch Label |
|----------|-------------|------------|--------------|
| Bottom face | `A` | A | B |
| Right face | `B` | B | A |
| Top face | `Y` | Y | X |
| Left face | `X` | X | Y |
| D-pad | `DPadUp`, `DPadDown`, `DPadLeft`, `DPadRight` | | |
| Bumpers | `LB`, `RB` | | |
| Triggers | `LT`, `RT` | | |
| Thumbsticks | `LS`, `RS` | | |
| Menu | `Select`, `Start` | | |
| Home | `Home` | | |

The app auto-detects your controller type and shows the correct labels in the UI. Use `--layout switch` to override.

## Key Names

### Letters & Numbers
`a`–`z`, `0`–`9`

### Special Keys
| Key | Aliases |
|-----|---------|
| `return` | `enter` |
| `tab` | |
| `space` | |
| `backspace` | |
| `delete` | `del` |
| `escape` | `esc` |
| `home` | |
| `end` | |
| `pageup` | |
| `pagedown` | |

### Arrow Keys
`up`, `down`, `left`, `right`

### Function Keys
`f1`–`f12`

### Punctuation
`-`, `=`, `[`, `]`, `;`, `'`, `,`, `.`, `/`, `\\`, `` ` ``

### Modifiers (as combo prefixes)
| Modifier | Aliases | macOS | Windows |
|----------|---------|-------|---------|
| `super` | | Cmd (⌘) | Ctrl |
| `ctrl` | `control` | Ctrl (⌃) | Ctrl |
| `alt` | `option` | Option (⌥) | Alt |
| `shift` | | Shift (⇧) | Shift |
| `cmd` | `command` | Cmd (⌘) | Win |

### Cross-Platform Modifier

Use `super` for the primary platform modifier — it maps to `Cmd` on macOS and `Ctrl` on Windows. This lets one profile work on both platforms:

```json
"A": "super+s"
```
→ `Cmd+S` on macOS, `Ctrl+S` on Windows.

## Key Combo Syntax

Combine modifiers with `+`:

```
super+s           → Cmd+S (Save)
super+shift+p     → Cmd+Shift+P (Command Palette)
ctrl+`            → Ctrl+` (Toggle Terminal)
alt+up            → Option+Up (Move Line Up)
```

## Controller Layout

The app auto-detects Xbox vs Nintendo Switch controllers based on the device name. The layout only affects display labels — profiles always use Xbox convention.

To override: `gamepad-mapper --layout switch`

Detected as Switch: controllers with "Nintendo", "Switch", "Pro Controller", or "Joy-Con" in the name.

## Creating Your Own Profile

```bash
# Start from an existing profile
cp profiles/default.json ~/Library/Application\ Support/gamepad-mapper/myprofile.json

# Edit it
gamepad-mapper --profile myprofile --edit

# Or use the interactive wizard
gamepad-mapper --profile myprofile --setup

# Validate before running
gamepad-mapper --profile myprofile --validate
```
