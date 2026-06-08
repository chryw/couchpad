# VS Code + controller (vibe coding guide)

Use a game controller to drive VS Code with GitHub Copilot Chat — no keyboard needed for the AI-assisted coding workflow.

## Why?

With Copilot Chat + voice input, you can code from your couch:
1. Speak your intent → Copilot writes the code
2. Review with DPad → Accept or reject
3. Save and run → Repeat

The controller handles navigation, confirmation, and app commands. Voice handles the "typing."

## Prerequisites

- [Gamepad Mapper](../README.md) installed and working
- [GitHub Copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot) extension
- [VS Code Speech](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-speech) extension (for voice input)

## Setup

```bash
gamepad-mapper --profile vscode
```

That's it — the `vscode` profile is built in. To customize it, save a local copy first:

```bash
gamepad-mapper --profile vscode --init
gamepad-mapper --profile vscode --edit
```

## Workflow

```
┌─────────────────────────────────────────────────────────┐
│  1. LT          → Open Copilot Chat                     │
│  2. Speak/Type  → Describe what you want                │
│  3. A (Enter)   → Send message                          │
│  4. DPad ↑↓     → Scroll through response               │
│  5. Y/RB (Tab)  → Accept suggestion                     │
│  6. Start       → Save file                             │
│  7. LS          → Undo (if something went wrong)        │
│  8. RT          → Quick Open (jump to changed files)    │
└─────────────────────────────────────────────────────────┘
```

## Keymap

### Base layer (normal presses)

> Shortcuts shown as macOS keys. On Windows, `Cmd` = `Ctrl` (the profile uses `super` internally).

| Button | Action | Shortcut |
|--------|--------|------------------|
| A | Enter | Send message / confirm |
| B | Escape | Dismiss panel / cancel |
| X | Space | Scroll down / toggle |
| Y | Tab | Accept Copilot suggestion |
| DPad | Arrow keys | Navigate code and chat |
| LB | Shift+Tab | Reject / previous suggestion |
| RB | Tab | Accept suggestion |
| **LT** | **Copilot Chat** | `Ctrl+Cmd+I` |
| RT | Quick Open | `Cmd+P` — jump to files |
| LS | Undo | `Cmd+Z` |
| RS | Redo | `Cmd+Shift+Z` |
| Select | Explorer | `Cmd+Shift+E` |
| Start | Save | `Cmd+S` |

### Layer (Home + button)

| Button | Action | Shortcut |
|--------|--------|------------------|
| A | Save | `Cmd+S` |
| B | Close tab | `Cmd+W` |
| X | Interrupt | `Ctrl+C` (stop terminal) |
| Y | Toggle terminal | `` Ctrl+` `` |
| DPad ↑↓ | Switch tabs | `Cmd+Shift+]` / `[` |
| DPad ←→ | Back / Forward | `Cmd+[` / `]` |
| LB | Undo | `Cmd+Z` |
| RB | Redo | `Cmd+Shift+Z` |
| LT | Command Palette | `Cmd+Shift+P` |
| RT | Run | `F5` |
| LS | Find | `Cmd+F` |
| RS | Toggle comment | `Cmd+/` |
| Select | Extensions | `Cmd+Shift+X` |
| Start | New file | `Cmd+N` |

## Voice input

### VS Code Speech (recommended)

The [VS Code Speech](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-speech) extension adds local, offline speech-to-text:

- **Mic icon** appears in the Copilot Chat input — click to start dictating
- **⌥⌘V** — keyboard shortcut to start dictation anywhere
- **"Hey Code"** — say it to open Copilot Chat hands-free

Settings to enable in VS Code (`settings.json`):
```json
{
  "accessibility.voice.keywordActivation": "chatInContext",
  "accessibility.voice.autoSynthesize": true,
  "accessibility.voice.speechTimeout": 1200
}
```

With `keywordActivation` enabled, just say **"Hey Code"** and start talking — no button press needed.

With `autoSynthesize` enabled, Copilot reads its responses aloud.

### macOS dictation (alternative)

If you prefer system-wide dictation:
1. System Settings → Keyboard → Dictation → Enable
2. Set a custom shortcut (e.g., `Ctrl+Shift+D`)
3. Add to your profile: `"Select": "ctrl+shift+d"` (layer mapping)

## Tips

- **Start simple:** Use base layer for chat interaction, layer for everything else
- **RB is your best friend:** Press it to accept Copilot's inline code suggestions
- **Quick Open (RT)** is great for jumping to files Copilot just created/modified
- **Undo (LS)** is always one press away if Copilot's edit isn't right
- **Voice + Controller** = fully hands-free coding from the couch
- **Controller layout:** Auto-detected. If wrong, use `--layout switch` to override
