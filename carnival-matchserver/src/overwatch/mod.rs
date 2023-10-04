use core::time;
use std::thread;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            FindWindowA, GetForegroundWindow, GetWindowLongPtrA, SetForegroundWindow,
            SetWindowLongPtrA, SetWindowPos, GWL_STYLE, SET_WINDOW_POS_FLAGS, WINDOW_STYLE,
            WS_BORDER, WS_CAPTION, WS_SIZEBOX,
        },
    },
};
use winput::Vk;

pub mod static_actions;
pub mod client_state;
pub mod game_state;
pub mod state_handler;
pub mod dyn_actions;

fn get_hwnd() -> Result<HWND, windows::core::Error> {
    let process_name: PCSTR = windows::core::s!("Overwatch");
    let hwnd = unsafe { FindWindowA(None, process_name) };
    log::debug!("{:?}", hwnd);
    if hwnd == HWND::default() {
        /* unsure of whats going on here because calling GetLastError()?
        and then unwrapping the result should return a panic but it doesnt? */
        //unsafe { GetLastError()?; } //
        panic!("Couldn't find Overwatch window. Is the client open and set to English?");
    }
    Ok(hwnd)
}

pub fn remove_window_decorations(
    hwnd: &HWND,
    style_to_remove: WINDOW_STYLE,
) -> Result<(), windows::core::Error> {
    // We get the hwnd for Overwatch which is enough to set the foreground window.
    // But for some reason, we need to get the engine window (child hwnd) or we run into
    // problems with cursor offset.
    unsafe { SetForegroundWindow(*hwnd) };
    thread::sleep(time::Duration::from_millis(200));
    let hwnd = unsafe { GetForegroundWindow() };
    let mut style = unsafe { GetWindowLongPtrA(hwnd, GWL_STYLE) };
    style &= !style_to_remove.0 as isize;

    // Send input to engine window, without this we have cursor offset issues.
    // I really really don't know why.
    winput::press(Vk::Control);
    unsafe { SetWindowLongPtrA(hwnd, GWL_STYLE, style) };
    winput::release(Vk::Control);

    // Force window repaint to avoid cursor offset issues.
    unsafe { SetWindowPos(hwnd, None, 0, 0, 1920, 1080, SET_WINDOW_POS_FLAGS(0))? };
    thread::sleep(time::Duration::from_millis(200));
    Ok(())
}

// The only way these errors are thrown is by targeting a window of an elevated process
// from a non-elevated process. In the context of Overwatch, this will never happen.
pub fn prelude() -> Result<(), windows::core::Error> {
    let hwnd = get_hwnd().unwrap();
    // Yes, these need to be set individually and in this order.
    remove_window_decorations(&hwnd, WS_CAPTION)?;
    remove_window_decorations(&hwnd, WS_SIZEBOX)?;
    remove_window_decorations(&hwnd, WS_BORDER)?;

    Ok(())
}
