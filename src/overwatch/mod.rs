use core::time;
use std::thread;
use windows::{Win32::{UI::WindowsAndMessaging::{FindWindowA, SetForegroundWindow, GetForegroundWindow, GetWindowLongPtrA, GWL_STYLE, SetWindowLongPtrA, SetWindowPos, SET_WINDOW_POS_FLAGS, WS_CAPTION, WS_SIZEBOX, WS_BORDER, WINDOW_STYLE, WS_MINIMIZE}, Foundation::{GetLastError, HWND}}, core::PCSTR};
use winput::Vk;

pub mod client_state;
pub mod game_state;
pub mod state_handler;



const NULL_HWND: HWND = HWND(0);

fn get_hwnd() -> Result<HWND, windows::core::Error> {
    let process_name: PCSTR = windows::core::s!("Untitled - Notepad");
    let hwnd = unsafe { FindWindowA(None, process_name) };
    println!("{:?}", hwnd);
    if hwnd == NULL_HWND {
        /* unsure of whats going on here because calling GetLastError()? 
        and then unwrapping the result should return a panic but it doesnt? */
        //unsafe { GetLastError()?; } // 
        panic!("Couldn't find Overwatch window. Is the client open and set to English?");
    }
    Ok(hwnd)
}

pub fn remove_window_decorations(hwnd: &HWND, style_to_remove: WINDOW_STYLE) -> Result<(), windows::core::Error> {
    // We get the hwnd for Overwatch which is enough to set the foreground window.
    // But for some reason, we need to get the engine window (child hwnd) or we run into
    // problems.
    unsafe { SetForegroundWindow(*hwnd) };
    thread::sleep(time::Duration::from_millis(200));
    let hwnd = unsafe { GetForegroundWindow() };
    let mut style = unsafe { GetWindowLongPtrA(hwnd, GWL_STYLE) };
    style &= !style_to_remove.0 as isize;

    // Send input to engine window, without this we have cursor offset issues.
    // I really really don't know why.
    winput::press(Vk::Control);
    unsafe { SetWindowLongPtrA(hwnd, GWL_STYLE, style as isize) };
    winput::release(Vk::Control);

    // Force window repaint to avoid cursor offset issues.
    unsafe { SetWindowPos(
        hwnd,
        None,
        1800,
        200,
        1920,
        1080,
        SET_WINDOW_POS_FLAGS(0),
    )?};
    thread::sleep(time::Duration::from_millis(200));
    Ok(())
}


pub fn client_prelude() {
    let hwnd = get_hwnd().unwrap();
    // Yes, these need to be set individually and in this order.
    remove_window_decorations(&hwnd, WS_CAPTION);
    remove_window_decorations(&hwnd, WS_SIZEBOX);
    remove_window_decorations(&hwnd, WS_BORDER);
}
