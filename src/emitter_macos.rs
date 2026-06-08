use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

/// Simulate a key press + release with optional modifier flags.
pub fn send_key(keycode: u16, flags: u64) {
    let source = match CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("  ⚠ Failed to create event source (check Accessibility permissions)");
            return;
        }
    };

    let key_down = match CGEvent::new_keyboard_event(source.clone(), keycode, true) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("  ⚠ Failed to create key event for keycode {}", keycode);
            return;
        }
    };
    let key_up = match CGEvent::new_keyboard_event(source, keycode, false) {
        Ok(e) => e,
        Err(_) => return,
    };

    if flags != 0 {
        let cg_flags = CGEventFlags::from_bits_truncate(flags);
        key_down.set_flags(cg_flags);
        key_up.set_flags(cg_flags);
    }

    key_down.post(CGEventTapLocation::HID);
    key_up.post(CGEventTapLocation::HID);
}
