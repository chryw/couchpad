USAGE:
  gamepad-mapper [OPTIONS]

OPTIONS:
  (no flags)             Start the mapper with the default profile
  --profile <name>       Use a specific profile
  --layout <type>        Override controller layout (xbox or switch, auto-detected)
  --init                 Create a new profile config file
  --setup                Interactive setup wizard (pick actions, press buttons)
  --edit                 Open config in your default editor
  --info                 Show controller info and keymap table
  --validate             Validate profile config (check buttons and keys)
  --test                 Test mode: show button presses without emitting keys
  --list                 List available profiles (pick to run)
  --config <path>        Use a custom config file path
  --help, -h             Show this help message

EXAMPLES:
  gamepad-mapper                        Start with default profile
  gamepad-mapper --profile vscode       Start with "vscode" profile
  gamepad-mapper --setup                Interactive wizard to map buttons
  gamepad-mapper --init                 Create default profile config
  gamepad-mapper --profile gaming --init Create a new "gaming" profile
  gamepad-mapper --edit                 Open config in default editor
  gamepad-mapper --test                 See raw button names from controller
  gamepad-mapper --info                 View current config and controller
  gamepad-mapper --validate             Check profile for errors
  gamepad-mapper --list                 Pick and run a profile interactively
