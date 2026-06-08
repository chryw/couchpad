USAGE:
  gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start with the default profile
  --profile <name>       Use a named profile
  --layout <type>        Override controller layout (xbox or switch, auto-detected)
  --init                 Create a new profile
  --setup                Interactive setup wizard (pick actions, press buttons)
  --edit                 Open profile in your default editor
  --info                 Show controller info and current mappings
  --validate             Check profile for errors (buttons and keys)
  --test                 Test mode: show button presses without emitting keys
  --list                 List available profiles (pick to run)
  --load <path>          Load a profile from a specific file path
  --help, -h             Show this help message

EXAMPLES:
  gamepad-mapper                        Start with default profile
  gamepad-mapper --profile vscode       Start with "vscode" profile
  gamepad-mapper --setup                Interactive wizard to map buttons
  gamepad-mapper --init                 Create default profile
  gamepad-mapper --profile gaming --init Create a new "gaming" profile
  gamepad-mapper --edit                 Open profile in default editor
  gamepad-mapper --test                 See raw button names from controller
  gamepad-mapper --info                 View current profile and controller
  gamepad-mapper --validate             Check profile for errors
  gamepad-mapper --list                 Pick and run a profile interactively
