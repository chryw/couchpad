# Profile reference

## Profile location

When you customize or create profiles, they're stored as JSON files in:
- **macOS:** `~/Library/Application Support/couchpad/<name>.json`
- **Windows:** `%APPDATA%/couchpad/<name>.json`

Local profiles take priority over built-in ones with the same name.

## Getting started

```bash
# Use a built-in profile immediately (no setup needed)
couchpad --profile vscode

# Or create your own from scratch
couchpad --init
couchpad --setup
```

## Built-in profiles

These profiles are embedded in the binary and work out of the box:

| Profile | Use case |
|---------|----------|
| `default` | Generic navigation — browse, scroll, tabs, clipboard |
| `vscode` | VS Code vibe coding with Copilot Chat ([guide](vscode-guide.md)) |

```bash
couchpad --profile vscode
couchpad --profile default
```

To customize a built-in profile, run `--init` to save a copy to disk, then `--edit`:

```bash
couchpad --profile vscode --init
couchpad --profile vscode --edit
```

Your local copy takes priority over the built-in version.

### `default.json` — Generic navigation

Designed for browsing, reading, and general app navigation. No app-specific shortcuts.

- **Base layer:** Enter, Escape, Space, Backspace, Arrows, Prev/Next tab, Page Up/Down, Home/End
- **Layer (Home+):** Paste, Copy, Ctrl+C, Select All, Tab switching, Back/Forward, Undo/Redo, Zoom, Find, Close tab

### `vscode.json` — VS Code vibe coding

Optimized for Copilot Chat-driven coding. The workflow: open chat → speak/type prompt → accept suggestions → save.

- **Base layer:** Enter, Escape, Tab (accept), Copilot Chat (LT), Quick Open (RT), Undo/Redo, Save
- **Layer (Home+):** Close tab, Terminal, Interrupt, Command Palette, Run (F5), Find, Switch tabs

## Profile format

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

## Layer system

Hold the layer button (default: `Home`) + press another button for alternate mappings. This doubles your available actions without needing more buttons.

Example: `Home + A` sends `super+s` (Save) while `A` alone sends `return`.

## Button names

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

## Key names

### Letters and numbers
`a`–`z`, `0`–`9`

### Special keys
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

### Arrow keys
`up`, `down`, `left`, `right`

### Function keys
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

### Cross-platform modifier

Use `super` for the primary platform modifier — it maps to `Cmd` on macOS and `Ctrl` on Windows. This lets one profile work on both platforms:

```json
"A": "super+s"
```
→ `Cmd+S` on macOS, `Ctrl+S` on Windows.

## Key combo syntax

Combine modifiers with `+`:

```
super+s           → Cmd+S (Save)
super+shift+p     → Cmd+Shift+P (Command Palette)
ctrl+`            → Ctrl+` (Toggle Terminal)
alt+up            → Option+Up (Move Line Up)
```

## Controller layout

The app auto-detects Xbox vs Nintendo Switch controllers based on the device name. The layout only affects display labels — profiles always use Xbox convention.

To override: `couchpad --layout switch`

Detected as Switch: controllers with "Nintendo", "Switch", "Pro Controller", or "Joy-Con" in the name.

## Creating your own profile

```bash
# Create a new empty profile
couchpad --profile myprofile --init

# Edit it
couchpad --profile myprofile --edit

# Or use the interactive wizard
couchpad --profile myprofile --setup

# Validate before running
couchpad --profile myprofile --validate
```
