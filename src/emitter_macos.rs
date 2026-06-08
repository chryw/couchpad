use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

/// Simulate a key press + release with optional modifier flags.
pub fn send_key(keycode: u16, flags: u64) {
    let source =
        CGEventSource::new(CGEventSourceStateID::HIDSystemState).expect("Failed to create event source");

    let key_down = CGEvent::new_keyboard_event(source.clone(), keycode, true)
        .expect("Failed to create key-down event");
    let key_up = CGEvent::new_keyboard_event(source, keycode, false)
        .expect("Failed to create key-up event");

    if flags != 0 {
        let cg_flags = CGEventFlags::from_bits_truncate(flags);
        key_down.set_flags(cg_flags);
        key_up.set_flags(cg_flags);
    }

    key_down.post(CGEventTapLocation::HID);
    key_up.post(CGEventTapLocation::HID);
}
