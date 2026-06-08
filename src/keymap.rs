use std::collections::HashMap;

/// Build the platform-appropriate keycode map.
/// On macOS: returns CGEvent keycodes.
/// On Windows: returns Windows Virtual-Key codes.
pub fn build_keycode_map() -> HashMap<&'static str, u16> {
    #[cfg(target_os = "macos")]
    { build_keycode_map_macos() }
    #[cfg(target_os = "windows")]
    { build_keycode_map_windows() }
}

/// macOS CGEvent keycodes
/// Reference: https://developer.apple.com/documentation/coregraphics/cgkeycode
#[cfg(target_os = "macos")]
fn build_keycode_map_macos() -> HashMap<&'static str, u16> {
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
    m.insert("delete", 0x75);
    m.insert("del", 0x75);
    m.insert("escape", 0x35);
    m.insert("esc", 0x35);
    m.insert("home", 0x73);
    m.insert("end", 0x77);
    m.insert("pageup", 0x74);
    m.insert("pagedown", 0x79);

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

/// Windows Virtual-Key codes
/// Reference: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
#[cfg(target_os = "windows")]
fn build_keycode_map_windows() -> HashMap<&'static str, u16> {
    let mut m = HashMap::new();

    // Letters A-Z (VK_A=0x41 through VK_Z=0x5A)
    for c in b'a'..=b'z' {
        let key: &'static str = match c {
            b'a' => "a", b'b' => "b", b'c' => "c", b'd' => "d",
            b'e' => "e", b'f' => "f", b'g' => "g", b'h' => "h",
            b'i' => "i", b'j' => "j", b'k' => "k", b'l' => "l",
            b'm' => "m", b'n' => "n", b'o' => "o", b'p' => "p",
            b'q' => "q", b'r' => "r", b's' => "s", b't' => "t",
            b'u' => "u", b'v' => "v", b'w' => "w", b'x' => "x",
            b'y' => "y", b'z' => "z", _ => unreachable!(),
        };
        m.insert(key, (0x41 + (c - b'a')) as u16);
    }

    // Numbers 0-9 (VK_0=0x30 through VK_9=0x39)
    for i in 0..=9u16 {
        let key: &'static str = match i {
            0 => "0", 1 => "1", 2 => "2", 3 => "3", 4 => "4",
            5 => "5", 6 => "6", 7 => "7", 8 => "8", 9 => "9",
            _ => unreachable!(),
        };
        m.insert(key, 0x30 + i);
    }

    // Special keys
    m.insert("return", 0x0D);  // VK_RETURN
    m.insert("enter", 0x0D);
    m.insert("tab", 0x09);     // VK_TAB
    m.insert("space", 0x20);   // VK_SPACE
    m.insert("backspace", 0x08); // VK_BACK
    m.insert("delete", 0x2E);  // VK_DELETE
    m.insert("del", 0x2E);
    m.insert("escape", 0x1B);  // VK_ESCAPE
    m.insert("esc", 0x1B);
    m.insert("home", 0x24);    // VK_HOME
    m.insert("end", 0x23);     // VK_END
    m.insert("pageup", 0x21);  // VK_PRIOR
    m.insert("pagedown", 0x22); // VK_NEXT

    // Modifiers (as keycodes, for standalone press)
    m.insert("shift", 0x10);   // VK_SHIFT
    m.insert("capslock", 0x14); // VK_CAPITAL
    m.insert("option", 0x12);  // VK_MENU (Alt)
    m.insert("alt", 0x12);
    m.insert("control", 0x11); // VK_CONTROL
    m.insert("ctrl", 0x11);
    m.insert("command", 0x5B); // VK_LWIN
    m.insert("cmd", 0x5B);

    // Arrow keys
    m.insert("left", 0x25);    // VK_LEFT
    m.insert("right", 0x27);   // VK_RIGHT
    m.insert("down", 0x28);    // VK_DOWN
    m.insert("up", 0x26);      // VK_UP

    // Function keys (VK_F1=0x70 through VK_F12=0x7B)
    m.insert("f1", 0x70);
    m.insert("f2", 0x71);
    m.insert("f3", 0x72);
    m.insert("f4", 0x73);
    m.insert("f5", 0x74);
    m.insert("f6", 0x75);
    m.insert("f7", 0x76);
    m.insert("f8", 0x77);
    m.insert("f9", 0x78);
    m.insert("f10", 0x79);
    m.insert("f11", 0x7A);
    m.insert("f12", 0x7B);

    // Punctuation (Windows VK codes for OEM keys)
    m.insert("minus", 0xBD);       // VK_OEM_MINUS
    m.insert("-", 0xBD);
    m.insert("equal", 0xBB);       // VK_OEM_PLUS (= key)
    m.insert("=", 0xBB);
    m.insert("leftbracket", 0xDB); // VK_OEM_4
    m.insert("[", 0xDB);
    m.insert("rightbracket", 0xDD); // VK_OEM_6
    m.insert("]", 0xDD);
    m.insert("semicolon", 0xBA);   // VK_OEM_1
    m.insert(";", 0xBA);
    m.insert("quote", 0xDE);       // VK_OEM_7
    m.insert("'", 0xDE);
    m.insert("comma", 0xBC);       // VK_OEM_COMMA
    m.insert(",", 0xBC);
    m.insert("period", 0xBE);      // VK_OEM_PERIOD
    m.insert(".", 0xBE);
    m.insert("slash", 0xBF);       // VK_OEM_2
    m.insert("/", 0xBF);
    m.insert("backslash", 0xDC);   // VK_OEM_5
    m.insert("\\", 0xDC);
    m.insert("grave", 0xC0);       // VK_OEM_3
    m.insert("`", 0xC0);

    m
}

/// Modifier flag constants (shared across platforms for config layer).
/// On Windows, the emitter interprets these same flag values.
pub mod modifiers {
    pub const SHIFT: u64 = 0x00020000;
    pub const CONTROL: u64 = 0x00040000;
    pub const OPTION: u64 = 0x00080000;  // Alt on Windows
    pub const COMMAND: u64 = 0x00100000; // Win key on Windows
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keycode_map_has_all_letters() {
        let map = build_keycode_map();
        let letters = ["a","b","c","d","e","f","g","h","i","j","k","l","m",
                       "n","o","p","q","r","s","t","u","v","w","x","y","z"];
        for key in letters {
            assert!(map.contains_key(key), "missing key: {}", key);
        }
    }

    #[test]
    fn keycode_map_has_all_digits() {
        let map = build_keycode_map();
        for i in 0..=9 {
            let key = &format!("{}", i);
            assert!(map.contains_key(key.as_str()), "missing key: {}", key);
        }
    }

    #[test]
    fn keycode_map_has_function_keys() {
        let map = build_keycode_map();
        for i in 1..=12 {
            let key = format!("f{}", i);
            assert!(map.contains_key(key.as_str()), "missing key: {}", key);
        }
    }

    #[test]
    fn keycode_map_has_essential_keys() {
        let map = build_keycode_map();
        let essentials = ["return", "enter", "tab", "space", "backspace", "delete",
                         "escape", "esc", "home", "end", "pageup", "pagedown",
                         "left", "right", "up", "down"];
        for key in essentials {
            assert!(map.contains_key(key), "missing essential key: {}", key);
        }
    }

    #[test]
    fn parse_simple_key() {
        let map = build_keycode_map();
        let result = parse_key_combo("return", &map);
        assert!(result.is_some());
        let (keycode, flags) = result.unwrap();
        assert_eq!(keycode, *map.get("return").unwrap());
        assert_eq!(flags, 0);
    }

    #[test]
    fn parse_key_with_modifier() {
        let map = build_keycode_map();
        let result = parse_key_combo("ctrl+c", &map);
        assert!(result.is_some());
        let (keycode, flags) = result.unwrap();
        assert_eq!(keycode, *map.get("c").unwrap());
        assert_eq!(flags, modifiers::CONTROL);
    }

    #[test]
    fn parse_key_with_multiple_modifiers() {
        let map = build_keycode_map();
        let result = parse_key_combo("ctrl+shift+p", &map);
        assert!(result.is_some());
        let (_, flags) = result.unwrap();
        assert_eq!(flags, modifiers::CONTROL | modifiers::SHIFT);
    }

    #[test]
    fn parse_super_resolves_to_platform_modifier() {
        let map = build_keycode_map();
        let result = parse_key_combo("super+p", &map);
        assert!(result.is_some());
        let (_, flags) = result.unwrap();
        if cfg!(target_os = "macos") {
            assert_eq!(flags, modifiers::COMMAND);
        } else {
            assert_eq!(flags, modifiers::CONTROL);
        }
    }

    #[test]
    fn parse_unknown_key_returns_none() {
        let map = build_keycode_map();
        assert!(parse_key_combo("nonexistent_key", &map).is_none());
    }

    #[test]
    fn parse_case_insensitive() {
        let map = build_keycode_map();
        let result = parse_key_combo("Ctrl+Shift+P", &map);
        assert!(result.is_some());
        let (_, flags) = result.unwrap();
        assert_eq!(flags, modifiers::CONTROL | modifiers::SHIFT);
    }

    #[test]
    fn aliases_map_to_same_keycode() {
        let map = build_keycode_map();
        assert_eq!(map.get("return"), map.get("enter"));
        assert_eq!(map.get("escape"), map.get("esc"));
        assert_eq!(map.get("delete"), map.get("del"));
        assert_eq!(map.get("alt"), map.get("option"));
        assert_eq!(map.get("ctrl"), map.get("control"));
        assert_eq!(map.get("cmd"), map.get("command"));
    }
}
