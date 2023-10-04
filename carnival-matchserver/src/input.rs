use std::{thread, time};

use winput::{Button, Mouse};

pub fn click(x: i32, y: i32) {
    if Mouse::set_position(x, y).is_ok() {
        thread::sleep(time::Duration::from_millis(200));
        winput::send(Button::Left);
    }
}

// Decoupled this from `actions.rs` in case we ever want to switch to 
// something other than winput.
pub fn type_str(input: &str, delay: u64) {
    input.chars().for_each(|char| {
        thread::sleep(time::Duration::from_millis(delay));
        winput::send_str(&char.to_string());
    });
    thread::sleep(time::Duration::from_millis(delay * 5));
}
