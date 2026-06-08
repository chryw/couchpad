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
    Option A — Use a built-in profile (quickest):
    $ gamepad-mapper --profile vscode    # Starts immediately

    Option B — Interactive wizard:
    $ gamepad-mapper --setup             # Pick actions, press buttons

    Option C — Manual setup:
    $ gamepad-mapper --init              # Create default profile
    $ gamepad-mapper --test              # Press buttons to see their names
    $ gamepad-mapper --edit              # Open profile to customize
    $ gamepad-mapper                     # Run with your custom profile

  Press Ctrl+C to stop.

  NOTE: On macOS, you'll be prompted to grant Accessibility permission on
  first run. See the README for details.
