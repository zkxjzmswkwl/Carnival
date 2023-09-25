use winput::{Mouse, Button, Vk};

pub fn click(x: i32, y: i32) {
    if Mouse::set_position(x, y).is_ok() {
        winput::send(Button::Left);
    }
}

