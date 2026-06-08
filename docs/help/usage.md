USAGE:
  gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start mapping with the default profile
  --edit                 Open your profile in $EDITOR to add/change mappings
  --help, -h             Show this help
  --info                 Show connected controller, active profile, and all mappings
  --init                 Create a new empty profile file to customize
  --layout <type>        Force controller labels to "xbox" or "switch" (default: auto-detected)
  --list                 Show all available profiles and pick one to run
  --load <path>          Load a profile from a specific file path instead of by name
  --profile <name>       Use a named profile (e.g. "vscode", "default")
  --setup                Step-by-step wizard: pick an action, then press the button to bind it
  --test                 Show raw button/axis events from your controller (no keys emitted)
  --validate             Check your profile for invalid button names or key combos

EXAMPLES:
  gamepad-mapper                        Start with default profile
  gamepad-mapper --profile vscode       Start with the "vscode" profile
  gamepad-mapper --setup                Guided wizard to create/edit mappings
  gamepad-mapper --init                 Create a blank profile to edit manually
  gamepad-mapper --test                 See what names your controller reports
  gamepad-mapper --validate             Catch typos in your profile
