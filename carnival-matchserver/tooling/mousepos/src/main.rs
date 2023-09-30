use std::thread::sleep;
use std::time::Duration;
use windows::core::Result;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

const DELAY: Duration = Duration::from_millis(25);
const TRIGGER_KEY: i32 = 'K' as i32;

#[derive(Default)]
struct CursorState {
    x: i32,
    y: i32,
}

impl CursorState {
    /// Update cursor coordinates.
    fn update(&mut self, new_pos: POINT) {
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

fn main() -> Result<()> {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    let mut state = CursorState::default();

    loop {
        // Check if the trigger key is pressed and try to get the cursor position.
        if unsafe { GetAsyncKeyState(TRIGGER_KEY) & 1 } != 0
            && unsafe { GetCursorPos(&mut cursor_pos).is_ok() }
        {
            state.update(cursor_pos);
            // Log the cursor state 
            println!("{{ x = {}, y = {}, delay = {}}},", state.x, state.y, DELAY.as_millis());
        }

        // Sleep for a defined delay to limit CPU usage.
        sleep(DELAY);
    }
}
