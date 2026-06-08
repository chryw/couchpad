// Windows key emitter using SendInput API
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_SHIFT, VK_MENU,
    VK_LWIN, VK_RETURN, VK_ESCAPE, VK_TAB, VK_SPACE, VK_BACK, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT,
    VK_DELETE, VK_HOME, VK_END, VK_PRIOR, VK_NEXT,
    VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
};
use std::collections::HashMap;
use std::mem;

pub fn build_vk_map() -> HashMap<&'static str, u16> {
    let mut m = HashMap::new();
    m.insert("return", VK_RETURN as u16);
    m.insert("enter", VK_RETURN as u16);
    m.insert("escape", VK_ESCAPE as u16);
    m.insert("esc", VK_ESCAPE as u16);
    m.insert("tab", VK_TAB as u16);
    m.insert("space", VK_SPACE as u16);
    m.insert("backspace", VK_BACK as u16);
    m.insert("delete", VK_DELETE as u16);
    m.insert("del", VK_DELETE as u16);
    m.insert("home", VK_HOME as u16);
    m.insert("end", VK_END as u16);
    m.insert("pageup", VK_PRIOR as u16);
    m.insert("pagedown", VK_NEXT as u16);
    m.insert("up", VK_UP as u16);
    m.insert("down", VK_DOWN as u16);
    m.insert("left", VK_LEFT as u16);
    m.insert("right", VK_RIGHT as u16);
    m.insert("f1", VK_F1 as u16);
    m.insert("f2", VK_F2 as u16);
    m.insert("f3", VK_F3 as u16);
    m.insert("f4", VK_F4 as u16);
    m.insert("f5", VK_F5 as u16);
    m.insert("f6", VK_F6 as u16);
    m.insert("f7", VK_F7 as u16);
    m.insert("f8", VK_F8 as u16);
    m.insert("f9", VK_F9 as u16);
    m.insert("f10", VK_F10 as u16);
    m.insert("f11", VK_F11 as u16);
    m.insert("f12", VK_F12 as u16);
    // Letters A-Z: VK codes are 0x41-0x5A
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
    // Numbers 0-9: VK codes are 0x30-0x39
    for i in 0..=9u16 {
        let key: &'static str = match i {
            0 => "0", 1 => "1", 2 => "2", 3 => "3", 4 => "4",
            5 => "5", 6 => "6", 7 => "7", 8 => "8", 9 => "9",
            _ => unreachable!(),
        };
        m.insert(key, 0x30 + i);
    }
    m
}

pub fn send_key(keycode: u16, flags: u64) {
    let mut inputs: Vec<INPUT> = Vec::new();

    // Press modifiers
    if flags & 0x00040000 != 0 { inputs.push(make_key_input(VK_CONTROL as u16, false)); }
    if flags & 0x00020000 != 0 { inputs.push(make_key_input(VK_SHIFT as u16, false)); }
    if flags & 0x00080000 != 0 { inputs.push(make_key_input(VK_MENU as u16, false)); }
    if flags & 0x00100000 != 0 { inputs.push(make_key_input(VK_LWIN as u16, false)); }

    // Press key
    inputs.push(make_key_input(keycode, false));
    // Release key
    inputs.push(make_key_input(keycode, true));

    // Release modifiers (reverse order)
    if flags & 0x00100000 != 0 { inputs.push(make_key_input(VK_LWIN as u16, true)); }
    if flags & 0x00080000 != 0 { inputs.push(make_key_input(VK_MENU as u16, true)); }
    if flags & 0x00020000 != 0 { inputs.push(make_key_input(VK_SHIFT as u16, true)); }
    if flags & 0x00040000 != 0 { inputs.push(make_key_input(VK_CONTROL as u16, true)); }

    let sent = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            mem::size_of::<INPUT>() as i32,
        )
    };

    if sent == 0 {
        eprintln!("  ⚠ SendInput failed (keycode: {})", keycode);
    }
}

fn make_key_input(vk: u16, key_up: bool) -> INPUT {
    let mut input: INPUT = unsafe { mem::zeroed() };
    input.type_ = INPUT_KEYBOARD;
    unsafe {
        let ki = input.u.ki_mut();
        ki.wVk = vk;
        ki.dwFlags = if key_up { KEYEVENTF_KEYUP } else { 0 };
    }
    input
}
