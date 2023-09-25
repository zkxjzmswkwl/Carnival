use core::time;
use std::thread;

use winapi::{
    shared::{minwindef::DWORD, windef},
    um::winuser::{
        self, GetForegroundWindow, GetWindowLongPtrA, SetForegroundWindow, SetWindowLongPtrA,
        SetWindowPos, GWL_STYLE, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE,
    },
};
use winput::Vk;

pub mod client_state;
pub mod game_state;
pub mod state_handler;

fn window_handle() -> windef::HWND {
    let window_handle = unsafe {
        winuser::FindWindowA(
            std::ptr::null::<i8>(),
            format!("Overwatch\0").as_ptr() as *const i8,
        )
    };
    if window_handle.is_null() {
        panic!("Couldn't find Overwatch window. Is the client open and set to English?");
    }
    return window_handle;
}

pub fn remove_window_decorations(style_to_remove: DWORD) {
    unsafe {
        let hwnd = window_handle();
        SetForegroundWindow(hwnd);
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

        // force window repaint to avoid cursor offset issues.
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED,
        );
        thread::sleep(time::Duration::from_millis(200));
    };
}
