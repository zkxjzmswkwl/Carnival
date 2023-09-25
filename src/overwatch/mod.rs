use core::time;
use std::thread;

use winapi::{
    shared::{minwindef::DWORD, windef},
    um::winuser::{
        self, GetForegroundWindow, GetWindowLongPtrA, SetForegroundWindow, SetWindowLongPtrA,
        SetWindowPos, GWL_STYLE, WS_CAPTION, WS_SIZEBOX, WS_BORDER,
    },
};
use winput::Vk;

pub mod client_state;
pub mod game_state;
pub mod state_handler;

fn get_hwnd() -> windef::HWND {
    let get_hwnd = unsafe {
        winuser::FindWindowA(
            std::ptr::null::<i8>(),
            format!("Overwatch\0").as_ptr() as *const i8,
        )
    };
    if get_hwnd.is_null() {
        panic!("Couldn't find Overwatch window. Is the client open and set to English?");
    }
    return get_hwnd;
}

pub fn remove_window_decorations(hwnd: &windef::HWND, style_to_remove: DWORD) {
    unsafe {
        SetForegroundWindow(*hwnd);
        thread::sleep(time::Duration::from_millis(200));
        // We get the hwnd for Overwatch which is enough to set the foreground window.
        // But for some reason, we need to get the engine window (child hwnd) or we run into
        // problems.
        let hwnd = GetForegroundWindow();
        let mut style = GetWindowLongPtrA(hwnd, GWL_STYLE) as u32;
        let remove_styles = style_to_remove;
        style &= !remove_styles;

        // Send input to engine window, without this we have cursor offset issues.
        // I really really don't know why.
        winput::press(Vk::Control);
        SetWindowLongPtrA(hwnd, GWL_STYLE, style as isize);
        winput::release(Vk::Control);

        // Force window repaint to avoid cursor offset issues.
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            1800,
            200,
            1920,
            1080,
            0,
        );
        thread::sleep(time::Duration::from_millis(200));
    };
}

pub fn client_prelude() {
    let hwnd = get_hwnd();
    // Yes, these need to be set individually and in this order.
    remove_window_decorations(&hwnd, WS_CAPTION);
    remove_window_decorations(&hwnd, WS_SIZEBOX);
    remove_window_decorations(&hwnd, WS_BORDER);
}
