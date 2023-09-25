use winput::{Button, Mouse};

pub fn click(x: i32, y: i32) {
    if Mouse::set_position(x, y).is_ok() {
        winput::send(Button::Left);
    }
}
