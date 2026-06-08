USAGE:
  gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start with the default profile
  --edit                 Open profile in your default editor
  --help, -h             Show this help message
  --info                 Show controller info and current mappings
  --init                 Create a new profile
  --layout <type>        Override controller layout (xbox or switch, auto-detected)
  --list                 List available profiles (pick to run)
  --load <path>          Load a profile from a specific file path
  --profile <name>       Use a named profile
  --setup                Interactive setup wizard (pick actions, press buttons)
  --test                 Test mode: show button presses without emitting keys
  --validate             Check profile for errors (buttons and keys)

EXAMPLES:
  gamepad-mapper                        Start with default profile
  gamepad-mapper --edit                 Open profile in default editor
  gamepad-mapper --info                 View current profile and controller
  gamepad-mapper --init                 Create default profile
  gamepad-mapper --list                 Pick and run a profile interactively
  gamepad-mapper --profile gaming --init Create a new "gaming" profile
  gamepad-mapper --profile vscode       Start with "vscode" profile
  gamepad-mapper --setup                Interactive wizard to map buttons
  gamepad-mapper --test                 See raw button names from controller
  gamepad-mapper --validate             Check profile for errors
