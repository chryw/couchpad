USAGE:
  couchpad [OPTIONS]

OPTIONS:
  (no flags)             Start mapping with the default profile
  --edit                 Open your profile in $EDITOR to add/change mappings
  --help, -h             Show this help
  --info                 Show connected controller, active profile, and all mappings
  --init                 Create a new empty profile file to customize
  --install              Install this binary to your PATH (run from Downloads)
  --layout <type>        Force controller labels to "xbox" or "switch" (default: auto-detected)
  --list                 Show all available profiles and pick one to run
  --load <path>          Load a profile from a specific file path instead of by name
  --profile <name>       Use a named profile (e.g. "vscode", "default")
  --setup                Step-by-step wizard: pick an action, then press the button to bind it
  --test                 Show raw button/axis events from your controller (no keys emitted)
  --validate             Check your profile for invalid button names or key combos
  --version, -V          Show version number

EXAMPLES:
  couchpad                        Start with default profile
  couchpad --profile vscode       Start with the "vscode" profile
  couchpad --setup                Guided wizard to create/edit mappings
  couchpad --init                 Create a blank profile to edit manually
  couchpad --test                 See what names your controller reports
  couchpad --validate             Catch typos in your profile
