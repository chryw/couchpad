PROFILE FORMAT:
  Profiles are stored in:
    macOS:   ~/Library/Application Support/gamepad-mapper/<name>.json
    Windows: %APPDATA%/gamepad-mapper/<name>.json

  Format:
  {
    "layer_button": "Home",
    "mappings": {
      "A": "return",
      "DPadUp": "up",
      "LT": "super+shift+p"
    },
    "layer_mappings": {
      "A": "super+s",
      "DPadUp": "alt+up"
    }
  }

  Hold the layer button + another button for alternate mappings.
  This doubles your available actions without needing more buttons.
