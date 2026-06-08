mod config;
mod keymap;

#[cfg(target_os = "macos")]
mod emitter_macos;
#[cfg(target_os = "windows")]
mod emitter_windows;

use gilrs::{Button, EventType, Gilrs};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Controller layout — determines display labels for face buttons.
/// Config always uses Xbox convention (A=bottom, B=right, X=left, Y=top).
/// Layout only affects what's shown in the UI so labels match the physical controller.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Layout {
    Xbox,
    Switch,
}

impl Layout {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "xbox" => Some(Layout::Xbox),
            "switch" | "nintendo" => Some(Layout::Switch),
            _ => None,
        }
    }

    /// Auto-detect layout from controller name
    fn detect(controller_name: &str) -> Self {
        let name = controller_name.to_lowercase();
        if name.contains("nintendo") || name.contains("switch")
            || name.contains("pro controller") || name.contains("joy-con")
        {
            Layout::Switch
        } else {
            Layout::Xbox
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Layout::Xbox => "xbox",
            Layout::Switch => "switch",
        }
    }
}

/// Detect layout from the first connected gamepad, or fall back to Xbox
fn detect_layout(gilrs: &Gilrs, layout_override: Option<Layout>) -> Layout {
    if let Some(l) = layout_override {
        return l;
    }
    for (_id, gamepad) in gilrs.gamepads() {
        return Layout::detect(gamepad.name());
    }
    Layout::Xbox
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse --profile flag
    let profile = args
        .windows(2)
        .find(|w| w[0] == "--profile")
        .map(|w| w[1].as_str().to_string());

    // Parse --config flag
    let config_path = args
        .windows(2)
        .find(|w| w[0] == "--config")
        .map(|w| PathBuf::from(&w[1]));

    // Parse --layout override (auto-detected from controller if not specified)
    let layout_override = args
        .windows(2)
        .find(|w| w[0] == "--layout")
        .and_then(|w| Layout::from_str(&w[1]));

    // Handle --help flag
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Handle --list flag
    if args.iter().any(|a| a == "--list") {
        match config::Config::list_profiles() {
            Ok(profiles) => {
                if profiles.is_empty() {
                    println!("No profiles found. Run with --init to create one.");
                } else {
                    println!("📋 Available profiles:\n");
                    for (i, p) in profiles.iter().enumerate() {
                        println!("   [{}] {}", i + 1, p);
                    }
                    println!("\n  Select a profile to run (1-{}), or press Enter to cancel:", profiles.len());

                    let mut input = String::new();
                    if std::io::stdin().read_line(&mut input).is_ok() {
                        let input = input.trim();
                        if input.is_empty() {
                            return;
                        }
                        // Try parsing as number
                        if let Ok(num) = input.parse::<usize>() {
                            if num >= 1 && num <= profiles.len() {
                                let selected = &profiles[num - 1];
                                println!("\n  Starting profile: {}\n", selected);
                                run_mapper(Some(selected.as_str()), None, layout_override);
                                return;
                            }
                        }
                        // Try matching by name
                        if let Some(matched) = profiles.iter().find(|p| p.as_str() == input) {
                            println!("\n  Starting profile: {}\n", matched);
                            run_mapper(Some(matched.as_str()), None, layout_override);
                            return;
                        }
                        eprintln!("  Invalid selection: {}", input);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }

    // Handle --init flag
    if args.iter().any(|a| a == "--init") {
        match config::Config::create_default(profile.as_deref()) {
            Ok(path) => {
                println!("✓ Created config at: {}", path.display());
                println!("  Edit it to customize your button mappings.");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Handle --info flag
    if args.iter().any(|a| a == "--info") {
        print_info(profile.as_deref(), config_path, layout_override);
        return;
    }

    // Handle --edit flag
    if args.iter().any(|a| a == "--edit") {
        open_config(profile.as_deref());
        return;
    }

    // Handle --setup flag
    if args.iter().any(|a| a == "--setup") {
        run_setup(profile.as_deref(), layout_override);
        return;
    }

    // Handle --test flag
    if args.iter().any(|a| a == "--test") {
        run_test_mode(profile.as_deref(), config_path, layout_override);
        return;
    }

    // Run the mapper
    run_mapper(profile.as_deref(), config_path, layout_override);
}

fn run_mapper(profile: Option<&str>, config_path: Option<PathBuf>, layout_override: Option<Layout>) {
    // Load config
    let cfg = match config::Config::load_profile(profile, config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize gamepad listener
    let mut gilrs = Gilrs::new().expect("Failed to initialize gamepad library");

    // Auto-detect layout from controller
    let layout = detect_layout(&gilrs, layout_override);

    // Build keycode lookup
    let keycode_map = keymap::build_keycode_map();

    // Parse layer button
    let layer_button = parse_button(&cfg.layer_button);

    // Pre-parse all mappings into (keycode, flags) pairs
    let mut button_actions: HashMap<Button, (u16, u64)> = HashMap::new();
    for (button_name, key_combo) in &cfg.mappings {
        let button = match parse_button(button_name) {
            Some(b) => b,
            None => {
                eprintln!("Warning: Unknown button '{}', skipping", button_name);
                continue;
            }
        };
        match keymap::parse_key_combo(key_combo, &keycode_map) {
            Some(action) => {
                button_actions.insert(button, action);
            }
            None => {
                eprintln!("Warning: Unknown key '{}', skipping", key_combo);
            }
        }
    }

    // Pre-parse layer mappings
    let mut layer_actions: HashMap<Button, (u16, u64)> = HashMap::new();
    for (button_name, key_combo) in &cfg.layer_mappings {
        let button = match parse_button(button_name) {
            Some(b) => b,
            None => {
                eprintln!("Warning: Unknown layer button '{}', skipping", button_name);
                continue;
            }
        };
        match keymap::parse_key_combo(key_combo, &keycode_map) {
            Some(action) => {
                layer_actions.insert(button, action);
            }
            None => {
                eprintln!("Warning: Unknown layer key '{}', skipping", key_combo);
            }
        }
    }

    let has_layer = !layer_actions.is_empty();

    // Print welcome banner
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  🎮 Gamepad Mapper                                  │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Profile: {:<41}│", profile.unwrap_or("default"));
    println!("│  Layout:  {:<41}│", layout.name());
    println!("│  Status:  ✓ Running                                 │");
    println!("│  Stop:    Ctrl+C                                    │");
    println!("└─────────────────────────────────────────────────────┘");
    println!();

    // Print connected gamepads
    let mut found_gamepad = false;
    for (_id, gamepad) in gilrs.gamepads() {
        println!("  🔗 Controller: {}", gamepad.name());
        println!("     OS name:   {}", gamepad.os_name());
        if let Some(map_name) = gamepad.map_name() {
            println!("     Mapping:   {}", map_name);
        }
        let uuid = gamepad.uuid();
        if uuid != [0u8; 16] {
            let uuid_str = uuid.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("     UUID:      {}", uuid_str);
        }
        found_gamepad = true;
    }
    if !found_gamepad {
        println!("  ⚠  No controllers detected. Connect one and restart.");
    }
    println!();

    // Print keymap table
    println!("  ┌──────────────────┬──────────────────────┬──────────────────────┐");
    println!("  │ Button           │ Normal               │ Layer ({:<5})        │", cfg.layer_button);
    println!("  ├──────────────────┼──────────────────────┼──────────────────────┤");

    let face = face_button_labels(layout);
    let all_buttons = [
        face[0], face[1], face[2], face[3],
        "DPadUp", "DPadDown", "DPadLeft", "DPadRight",
        "LB", "RB", "LT", "RT",
        "LS", "RS", "Select", "Start",
    ];

    for name in &all_buttons {
        let button = parse_button(name);
        let normal = button.and_then(|b| {
            cfg.mappings.iter().find(|(k, _)| parse_button(k) == Some(b)).map(|(_, v)| v.as_str())
        }).unwrap_or("—");
        let layer = button.and_then(|b| {
            cfg.layer_mappings.iter().find(|(k, _)| parse_button(k) == Some(b)).map(|(_, v)| v.as_str())
        }).unwrap_or("—");
        println!("  │ {:<16} │ {:<20} │ {:<20} │", name, normal, layer);
    }

    println!("  └──────────────────┴──────────────────────┴──────────────────────┘");
    if has_layer {
        println!("\n  Hold [{}] + button for layer actions.", cfg.layer_button);
    }
    println!();

    // Main event loop
    let mut layer_active = false;
    loop {
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    if layer_button == Some(button) {
                        layer_active = true;
                        continue;
                    }

                    let action = if layer_active {
                        layer_actions.get(&button).or(button_actions.get(&button))
                    } else {
                        button_actions.get(&button)
                    };

                    if let Some(&(keycode, flags)) = action {
                        let layer_indicator = if layer_active { " [layer]" } else { "" };
                        println!("  {} → sending key{}", button_display_name(button, layout), layer_indicator);
                        emit_key(keycode, flags);
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if layer_button == Some(button) {
                        layer_active = false;
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[cfg(target_os = "macos")]
fn emit_key(keycode: u16, flags: u64) {
    emitter_macos::send_key(keycode, flags);
}

#[cfg(target_os = "windows")]
fn emit_key(keycode: u16, flags: u64) {
    emitter_windows::send_key(keycode, flags);
}

fn parse_button(name: &str) -> Option<Button> {
    match name {
        "South" | "A" => Some(Button::South),
        "East" | "B" => Some(Button::East),
        "North" | "Y" => Some(Button::North),
        "West" | "X" => Some(Button::West),
        "DPadUp" => Some(Button::DPadUp),
        "DPadDown" => Some(Button::DPadDown),
        "DPadLeft" => Some(Button::DPadLeft),
        "DPadRight" => Some(Button::DPadRight),
        "LeftTrigger" | "LB" => Some(Button::LeftTrigger),
        "RightTrigger" | "RB" => Some(Button::RightTrigger),
        "LeftTrigger2" | "LT" => Some(Button::LeftTrigger2),
        "RightTrigger2" | "RT" => Some(Button::RightTrigger2),
        "LeftThumb" | "LS" => Some(Button::LeftThumb),
        "RightThumb" | "RS" => Some(Button::RightThumb),
        "Select" => Some(Button::Select),
        "Start" => Some(Button::Start),
        "Mode" | "Home" => Some(Button::Mode),
        _ => None,
    }
}

/// Get the display name for a button based on controller layout.
/// Config always uses Xbox names; this translates for display purposes.
fn button_display_name(button: Button, layout: Layout) -> &'static str {
    match button {
        Button::South => match layout { Layout::Xbox => "A", Layout::Switch => "B" },
        Button::East => match layout { Layout::Xbox => "B", Layout::Switch => "A" },
        Button::North => match layout { Layout::Xbox => "Y", Layout::Switch => "X" },
        Button::West => match layout { Layout::Xbox => "X", Layout::Switch => "Y" },
        Button::DPadUp => "DPadUp",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::DPadRight => "DPadRight",
        Button::LeftTrigger => "LB",
        Button::RightTrigger => "RB",
        Button::LeftTrigger2 => "LT",
        Button::RightTrigger2 => "RT",
        Button::LeftThumb => "LS",
        Button::RightThumb => "RS",
        Button::Select => "Select",
        Button::Start => "Start",
        Button::Mode => "Home",
        _ => "Unknown",
    }
}

/// Face button display labels in the order: bottom, right, top, left
fn face_button_labels(layout: Layout) -> [&'static str; 4] {
    match layout {
        Layout::Xbox => ["A", "B", "Y", "X"],
        Layout::Switch => ["B", "A", "X", "Y"],
    }
}

fn run_test_mode(profile: Option<&str>, config_path: Option<PathBuf>, layout_override: Option<Layout>) {
    let mut gilrs = Gilrs::new().expect("Failed to initialize gamepad library");

    let layout = detect_layout(&gilrs, layout_override);

    // Load config to show mapped keys
    let mappings: HashMap<String, String> = match config::Config::load_profile(profile, config_path) {
        Ok(cfg) => cfg.mappings,
        Err(_) => HashMap::new(),
    };

    println!("🧪 Test Mode — press buttons on your controller");
    println!("   Profile: {}", profile.unwrap_or("default"));
    println!("   Layout:  {}", layout.name());
    println!("   Shows raw button/axis names and their mapped keys");
    println!("   Press Ctrl+C to stop\n");

    for (_id, gamepad) in gilrs.gamepads() {
        println!("   Found: {} ({})", gamepad.name(), gamepad.os_name());
    }
    println!();

    loop {
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    let display = button_display_name(button, layout);
                    let internal_name = format!("{:?}", button);
                    let mapped = mappings.get(&internal_name)
                        .or_else(|| mappings.get(display))
                        .map(|k| format!(" → [{}]", k))
                        .unwrap_or_else(|| " (unmapped)".to_string());
                    println!("  ▶ {}{}", display, mapped);
                }
                EventType::ButtonReleased(button, _) => {
                    println!("  ◀ {} released", button_display_name(button, layout));
                }
                EventType::AxisChanged(axis, value, _) => {
                    if value.abs() > 0.5 {
                        println!("  ↔ Axis {:?} = {:.2}", axis, value);
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

fn print_info(profile: Option<&str>, config_path: Option<PathBuf>, layout_override: Option<Layout>) {
    let gilrs = Gilrs::new().expect("Failed to initialize gamepad library");

    let layout = detect_layout(&gilrs, layout_override);
    let cfg = config::Config::load_profile(profile, config_path).ok();
    let profile_name = profile.unwrap_or("default");

    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  🎮 Gamepad Mapper                                  │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Profile: {:<41}│", profile_name);
    println!("│  Layout:  {:<41}│", layout.name());
    println!("│  Status:  idle (not running)                        │");
    println!("└─────────────────────────────────────────────────────┘");
    println!();

    let mut found_gamepad = false;
    for (_id, gamepad) in gilrs.gamepads() {
        println!("  🔗 Controller: {}", gamepad.name());
        println!("     OS name:   {}", gamepad.os_name());
        if let Some(map_name) = gamepad.map_name() {
            println!("     Mapping:   {}", map_name);
        }
        let uuid = gamepad.uuid();
        if uuid != [0u8; 16] {
            let uuid_str = uuid.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("     UUID:      {}", uuid_str);
        }
        found_gamepad = true;
    }
    if !found_gamepad {
        println!("  ⚠  No controllers detected.");
    }
    println!();

    // Controller layout diagram with layout-correct labels
    let face = face_button_labels(layout);
    println!("  Controller Layout ({}):", layout.name());
    println!("  ┌──────────────────────────────────────────┐");
    println!("  │              [LB]      [RB]               │");
    println!("  │              [LT]      [RT]               │");
    println!("  │                                           │");
    println!("  │       ┌───┐                 [{}]           │", face[2]);
    println!("  │       │ ↑ │              [{}]   [{}]        │", face[3], face[1]);
    println!("  │    ┌──┼───┼──┐              [{}]           │", face[0]);
    println!("  │    │ ←│   │→ │                            │");
    println!("  │    └──┼───┼──┘                            │");
    println!("  │       │ ↓ │     [Select] [Start]          │");
    println!("  │       └───┘                               │");
    println!("  │         (●)       [Home]       (●)        │");
    println!("  │         LS                      RS        │");
    println!("  └──────────────────────────────────────────┘");
    println!();

    if let Some(cfg) = &cfg {
        println!("  ┌──────────────────┬──────────────────────┬──────────────────────┐");
        println!("  │ Button           │ Normal               │ Layer ({:<5})        │", cfg.layer_button);
        println!("  ├──────────────────┼──────────────────────┼──────────────────────┤");

        let all_buttons = [
            face[0], face[1], face[2], face[3],
            "DPadUp", "DPadDown", "DPadLeft", "DPadRight",
            "LB", "RB", "LT", "RT",
            "LS", "RS", "Select", "Start",
        ];

        for name in &all_buttons {
            let button = parse_button(name);
            let normal = button.and_then(|b| {
                cfg.mappings.iter().find(|(k, _)| parse_button(k) == Some(b)).map(|(_, v)| v.as_str())
            }).unwrap_or("—");
            let layer = button.and_then(|b| {
                cfg.layer_mappings.iter().find(|(k, _)| parse_button(k) == Some(b)).map(|(_, v)| v.as_str())
            }).unwrap_or("—");
            println!("  │ {:<16} │ {:<20} │ {:<20} │", name, normal, layer);
        }

        println!("  └──────────────────┴──────────────────────┴──────────────────────┘");
        if !cfg.layer_mappings.is_empty() {
            println!("\n  Hold [{}] + button for layer actions.", cfg.layer_button);
        }
    } else {
        println!("  No config loaded for profile '{}'.", profile_name);
    }
}

fn print_help() {
    println!(r#"🎮 Gamepad Mapper
   Map game controller buttons to keyboard shortcuts.

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
   gamepad-mapper --list                 Pick and run a profile interactively

CONFIG:
   Profiles are stored in:
     macOS:   ~/Library/Application Support/gamepad-mapper/<profile>.json
     Windows: %APPDATA%/gamepad-mapper/<profile>.json

   Config format:
   {{
     "layer_button": "Home",
     "mappings": {{
       "A": "return",
       "DPadUp": "up",
       "LT": "super+shift+p"
     }},
     "layer_mappings": {{
       "A": "super+s",
       "DPadUp": "alt+up"
     }}
   }}

BUTTON NAMES:
   A, B, X, Y             Face buttons
   DPadUp/Down/Left/Right D-pad
   LB, RB                 Bumpers (shoulders)
   LT, RT                 Triggers
   LS, RS                 Thumbstick clicks
   Select, Start          Menu buttons
   Home                   Home/Mode button

KEY NAMES:
   Letters: a-z            Numbers: 0-9
   Modifiers: super, ctrl, shift, alt, cmd
   Special: return, escape, tab, space, backspace
   Arrows: up, down, left, right
   Function: f1-f12
   Punctuation: [, ], -, =, ;, ', ,, ., /, \, `

   "super" maps to Cmd on macOS and Ctrl on Windows.
   Combine with +: "super+shift+p", "alt+up", "ctrl+cmd+i"

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

NOTES:
   • Use --setup for an interactive guided experience (no manual editing)
   • Use --test to discover button names for your specific controller
   • Use --info to verify your config and see the full keymap at a glance
   • Use --edit to open config in your default editor
   • Use --list to pick and run a profile interactively
   • Use --help to see this guide again
   • Hold the layer button + another button for alternate mappings
"#);
}

fn open_config(profile: Option<&str>) {
    let path = config::profile_path(profile.unwrap_or("default"));

    if !path.exists() {
        eprintln!("Config not found: {}", path.display());
        eprintln!("Run with --init to create it first.");
        std::process::exit(1);
    }

    println!("  Opening: {}", path.display());

    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(&path).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", ""]).arg(&path).status()
    } else {
        Command::new("xdg-open").arg(&path).status()
    };

    match result {
        Ok(status) if status.success() => {}
        Ok(_) => eprintln!("  Editor exited with an error."),
        Err(e) => eprintln!("  Failed to open editor: {}", e),
    }
}

fn run_setup(profile: Option<&str>, layout_override: Option<Layout>) {
    use std::io::{self, Write};

    // Known actions with their key combos
    let actions: Vec<(&str, &str, &str)> = vec![
        // (name, key combo, description)
        ("Save", "super+s", "Save current file"),
        ("Undo", "super+z", "Undo last action"),
        ("Redo", "super+shift+z", "Redo last action"),
        ("Command Palette", "super+shift+p", "Open command palette"),
        ("Quick Open", "super+p", "Quick open file by name"),
        ("Copilot Chat", "ctrl+super+i", "Toggle Copilot Chat panel"),
        ("Accept Suggestion", "tab", "Accept Copilot/autocomplete suggestion"),
        ("Reject Suggestion", "escape", "Dismiss suggestion or dialog"),
        ("Explorer", "super+shift+e", "Show file explorer sidebar"),
        ("Extensions", "super+shift+x", "Show extensions panel"),
        ("Toggle Terminal", "ctrl+`", "Show/hide integrated terminal"),
        ("New File", "super+n", "Create a new file"),
        ("Close Tab", "super+w", "Close current tab"),
        ("Next Tab", "super+shift+]", "Switch to next editor tab"),
        ("Prev Tab", "super+shift+[", "Switch to previous editor tab"),
        ("Move Line Up", "alt+up", "Move current line up"),
        ("Move Line Down", "alt+down", "Move current line down"),
        ("Navigate Up", "up", "Move cursor up"),
        ("Navigate Down", "down", "Move cursor down"),
        ("Navigate Left", "left", "Move cursor left"),
        ("Navigate Right", "right", "Move cursor right"),
        ("Confirm", "return", "Confirm / Enter"),
        ("Cancel", "escape", "Cancel / Escape"),
        ("Space", "space", "Space key"),
        ("Indent", "tab", "Tab / Indent"),
        ("Outdent", "shift+tab", "Shift+Tab / Outdent"),
    ];

    let mut gilrs = Gilrs::new().expect("Failed to initialize gamepad library");

    let layout = detect_layout(&gilrs, layout_override);

    // Drain any buffered events
    while gilrs.next_event().is_some() {}

    let profile_name = profile.unwrap_or("default");

    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  🎮 Gamepad Mapper — Interactive Setup               │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Profile: {:<41}│", profile_name);
    println!("└─────────────────────────────────────────────────────┘");
    println!();

    // Check controller
    let mut found = false;
    for (_id, gamepad) in gilrs.gamepads() {
        println!("  🔗 Controller: {}", gamepad.name());
        found = true;
    }
    if !found {
        eprintln!("\n  ⚠  No controller detected! Connect one and try again.");
        std::process::exit(1);
    }
    println!();

    println!("  Pick actions to map, then press a controller button.");
    println!("  Type 's' to skip, 'q' to finish and save.\n");

    println!("  Available actions:");
    println!("  ─────────────────────────────────────────────────────");
    for (i, (name, combo, desc)) in actions.iter().enumerate() {
        println!("  [{:>2}] {:<22} ({:<18}) {}", i + 1, name, combo, desc);
    }
    println!();

    let mut mappings: HashMap<String, String> = HashMap::new();

    loop {
        print!("  Select action # (or 'q' to finish): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();

        if input == "q" || input == "Q" {
            break;
        }

        let idx: usize = match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= actions.len() => n - 1,
            _ => {
                println!("  Invalid selection. Try again.\n");
                continue;
            }
        };

        let (name, combo, _) = actions[idx];
        println!("\n  → Mapping: {} ({})", name, combo);
        println!("    Press the controller button you want to use (or 's' to skip)...");

        // Wait for button press
        let mut skipped = false;

        // Check for 's' key in a non-blocking way — actually we need to
        // poll both stdin and gilrs. For simplicity, just poll gilrs with a timeout.
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(15);
        let mut detected_button: Option<Button> = None;

        loop {
            while let Some(event) = gilrs.next_event() {
                if let EventType::ButtonPressed(button, _) = event.event {
                    detected_button = Some(button);
                    break;
                }
            }
            if detected_button.is_some() {
                break;
            }
            if start.elapsed() > timeout {
                println!("    ⏱  Timed out (15s). Skipping.\n");
                skipped = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        if skipped {
            continue;
        }

        if let Some(button) = detected_button {
            let btn_name = button_display_name(button, layout).to_string();
            println!("    ✓ Detected: {}", btn_name);
            println!("    ✓ Mapped: {} → {}\n", btn_name, combo);
            mappings.insert(btn_name, combo.to_string());
        }
    }

    if mappings.is_empty() {
        println!("\n  No mappings created. Exiting.");
        return;
    }

    // Save the config
    let config_path = config::profile_path(profile_name);
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let cfg = config::Config {
        mappings,
        layer_mappings: HashMap::new(),
        layer_button: "Home".to_string(),
    };

    let json = serde_json::to_string_pretty(&cfg).expect("Failed to serialize config");
    std::fs::write(&config_path, json).expect("Failed to write config");

    println!("\n  ✓ Saved {} mapping(s) to: {}", cfg.mappings.len(), config_path.display());
    println!("    Run with: gamepad-mapper --profile {}", profile_name);
}