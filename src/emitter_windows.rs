// Windows key emitter using SendInput API
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_SHIFT, VK_MENU,
    VK_LWIN, VK_RETURN, VK_ESCAPE, VK_TAB, VK_SPACE, VK_BACK, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT,
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
    m.insert("up", VK_UP as u16);
    m.insert("down", VK_DOWN as u16);
    m.insert("left", VK_LEFT as u16);
    m.insert("right", VK_RIGHT as u16);
    // Letters A-Z: VK codes are 0x41-0x5A
    for (i, c) in ('a'..='z').enumerate() {
        let key: String = c.to_string();
        m.insert(Box::leak(key.into_boxed_str()), (0x41 + i) as u16);
    }
    // Numbers 0-9: VK codes are 0x30-0x39
    for i in 0..=9u16 {
        let key: String = i.to_string();
        m.insert(Box::leak(key.into_boxed_str()), 0x30 + i);
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

    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            mem::size_of::<INPUT>() as i32,
        );
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
