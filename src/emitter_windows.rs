// Windows key emitter using SendInput API
// Keycodes come from keymap::build_keycode_map() which returns Windows VK codes on this platform.
// Modifier flags use the same constants as keymap::modifiers.
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_SHIFT, VK_MENU,
    VK_LWIN,
};
use std::mem;

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
