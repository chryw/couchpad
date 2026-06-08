GETTING STARTED:

  Step 1: Pair your Bluetooth controller
  ─────────────────────────────────────────
  macOS:
    1. Put your controller in pairing mode (usually hold Home/Mode
       until LED blinks rapidly)
    2. Open System Settings → Bluetooth
    3. Find your controller in the device list and click "Connect"
    4. Wait for status to show "Connected"

  Windows:
    1. Put your controller in pairing mode
    2. Open Settings → Bluetooth & devices → Add device
    3. Select your controller and pair

  Step 2: Set up gamepad-mapper
  ─────────────────────────────────────────
    Option A — Interactive wizard (recommended for beginners):
    $ gamepad-mapper --setup             # Pick actions, press buttons

    Option B — Manual setup:
    $ gamepad-mapper --init              # Create default config
    $ gamepad-mapper --info              # Verify controller is detected
    $ gamepad-mapper --test              # Press buttons to see their names
    $ gamepad-mapper --edit              # Open config to customize

  Step 3: Customize your mappings
  ─────────────────────────────────────────
    Edit the config file shown by --info. Map button names (from --test)
    to key combos you want to emit.

  Step 4: Grant permissions (macOS only)
  ─────────────────────────────────────────
    The first time you run, macOS will ask for Accessibility permission.
    Go to: System Settings → Privacy & Security → Accessibility
    Toggle ON your terminal app (Terminal, iTerm2, or VS Code).

  Step 5: Run!
  ─────────────────────────────────────────
    $ gamepad-mapper                     # Start with default profile
    $ gamepad-mapper --profile vscode    # Start with a named profile
    Press Ctrl+C to stop.
