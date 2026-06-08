use std::collections::HashMap;

/// Maps a key name string (from config) to a macOS CGEvent keycode.
/// Reference: https://developer.apple.com/documentation/coregraphics/cgkeycode
pub fn build_keycode_map() -> HashMap<&'static str, u16> {
    let mut m = HashMap::new();

    // Letters
    m.insert("a", 0x00);
    m.insert("s", 0x01);
    m.insert("d", 0x02);
    m.insert("f", 0x03);
    m.insert("h", 0x04);
    m.insert("g", 0x05);
    m.insert("z", 0x06);
    m.insert("x", 0x07);
    m.insert("c", 0x08);
    m.insert("v", 0x09);
    m.insert("b", 0x0B);
    m.insert("q", 0x0C);
    m.insert("w", 0x0D);
    m.insert("e", 0x0E);
    m.insert("r", 0x0F);
    m.insert("y", 0x10);
    m.insert("t", 0x11);
    m.insert("1", 0x12);
    m.insert("2", 0x13);
    m.insert("3", 0x14);
    m.insert("4", 0x15);
    m.insert("6", 0x16);
    m.insert("5", 0x17);
    m.insert("9", 0x19);
    m.insert("7", 0x1A);
    m.insert("8", 0x1C);
    m.insert("0", 0x1D);
    m.insert("o", 0x1F);
    m.insert("u", 0x20);
    m.insert("i", 0x22);
    m.insert("p", 0x23);
    m.insert("l", 0x25);
    m.insert("j", 0x26);
    m.insert("k", 0x28);
    m.insert("n", 0x2D);
    m.insert("m", 0x2E);

    // Special keys
    m.insert("return", 0x24);
    m.insert("enter", 0x24);
    m.insert("tab", 0x30);
    m.insert("space", 0x31);
    m.insert("backspace", 0x33);
    m.insert("escape", 0x35);
    m.insert("esc", 0x35);

    // Modifiers (as keycodes, for standalone press)
    m.insert("shift", 0x38);
    m.insert("capslock", 0x39);
    m.insert("option", 0x3A);
    m.insert("alt", 0x3A);
    m.insert("control", 0x3B);
    m.insert("ctrl", 0x3B);
    m.insert("command", 0x37);
    m.insert("cmd", 0x37);

    // Arrow keys
    m.insert("left", 0x7B);
    m.insert("right", 0x7C);
    m.insert("down", 0x7D);
    m.insert("up", 0x7E);

    // Function keys
    m.insert("f1", 0x7A);
    m.insert("f2", 0x78);
    m.insert("f3", 0x63);
    m.insert("f4", 0x76);
    m.insert("f5", 0x60);
    m.insert("f6", 0x61);
    m.insert("f7", 0x62);
    m.insert("f8", 0x64);
    m.insert("f9", 0x65);
    m.insert("f10", 0x6D);
    m.insert("f11", 0x67);
    m.insert("f12", 0x6F);

    // Punctuation
    m.insert("minus", 0x1B);
    m.insert("-", 0x1B);
    m.insert("equal", 0x18);
    m.insert("=", 0x18);
    m.insert("leftbracket", 0x21);
    m.insert("[", 0x21);
    m.insert("rightbracket", 0x1E);
    m.insert("]", 0x1E);
    m.insert("semicolon", 0x29);
    m.insert(";", 0x29);
    m.insert("quote", 0x27);
    m.insert("'", 0x27);
    m.insert("comma", 0x2B);
    m.insert(",", 0x2B);
    m.insert("period", 0x2F);
    m.insert(".", 0x2F);
    m.insert("slash", 0x2C);
    m.insert("/", 0x2C);
    m.insert("backslash", 0x2A);
    m.insert("\\", 0x2A);
    m.insert("grave", 0x32);
    m.insert("`", 0x32);

    m
}

/// Modifier flag constants for CGEvent
pub mod modifiers {
    pub const SHIFT: u64 = 0x00020000; // kCGEventFlagMaskShift
    pub const CONTROL: u64 = 0x00040000; // kCGEventFlagMaskControl
    pub const OPTION: u64 = 0x00080000; // kCGEventFlagMaskAlternate
    pub const COMMAND: u64 = 0x00100000; // kCGEventFlagMaskCommand
}

/// Parse a key string like "ctrl+shift+p" into (keycode, modifier_flags)
/// "super" resolves to Command on macOS and Control on Windows.
pub fn parse_key_combo(combo: &str, keycode_map: &HashMap<&str, u16>) -> Option<(u16, u64)> {
    let parts: Vec<&str> = combo.split('+').map(|s| s.trim()).collect();
    let mut flags: u64 = 0;
    let mut key: Option<u16> = None;

    for part in &parts {
        let lower = part.to_lowercase();
        match lower.as_str() {
            "ctrl" | "control" => flags |= modifiers::CONTROL,
            "shift" => flags |= modifiers::SHIFT,
            "alt" | "option" => flags |= modifiers::OPTION,
            "cmd" | "command" => flags |= modifiers::COMMAND,
            "super" => flags |= super_modifier(),
            other => {
                key = keycode_map.get(other).copied();
            }
        }
    }

    key.map(|k| (k, flags))
}

/// Returns the platform-appropriate "super" modifier:
/// Command on macOS, Control on Windows.
fn super_modifier() -> u64 {
    if cfg!(target_os = "macos") {
        modifiers::COMMAND
    } else {
        modifiers::CONTROL
    }
}
