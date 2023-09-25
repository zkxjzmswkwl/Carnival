use std::{thread, time};

use winput::{Button, Mouse};

pub fn click(x: i32, y: i32) {
    if Mouse::set_position(x, y).is_ok() {
        thread::sleep(time::Duration::from_millis(200));
        winput::send(Button::Left);
    }
}
