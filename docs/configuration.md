# Configuration

## Profile Location

Profiles are JSON files stored in:
- **macOS:** `~/Library/Application Support/gamepad-mapper/<profile>.json`
- **Windows:** `%APPDATA%/gamepad-mapper/<profile>.json`

Use `--profile <name>` to load a specific profile, or omit it to use `default.json`.

## Config Format

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

### Fields

| Field | Description |
|-------|-------------|
| `layer_button` | Button that activates layer 2 when held (default: `"Home"`) |
| `mappings` | Button → key combo mappings for the base layer |
| `layer_mappings` | Button → key combo mappings when layer button is held |

## Layer System

Hold the layer button (default: `Home`) + press another button for alternate mappings. This doubles your available actions without needing more buttons.

Example: `Home + A` sends `super+s` (Save) while `A` alone sends `return`.

## Button Names

Config files always use Xbox-convention names regardless of which controller is connected:

| Position | Config Name | Xbox Label | Switch Label |
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
| `cmd` | `command` | Cmd (⌘) | Cmd (⌘) |

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

The app auto-detects Xbox vs Nintendo Switch controllers based on the device name. The layout only affects display labels — config always uses Xbox convention.

To override: `gamepad-mapper --layout switch`

Detected as Switch: controllers with "Nintendo", "Switch", "Pro Controller", or "Joy-Con" in the name.
