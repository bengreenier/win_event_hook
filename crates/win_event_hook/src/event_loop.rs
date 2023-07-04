use tracing::trace;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, WM_QUIT},
};

/// Runs a windows event loop for pressing messages using [`GetMessageW`] and [`DispatchMessageW`].
pub unsafe fn run_event_loop() {
    trace!("starting event_loop");
    let mut message = MSG::default();
    while GetMessageW(&mut message, HWND(0), 0, 0).into() {
        if message.message == WM_QUIT {
            break;
        }
        DispatchMessageW(&message);
    }
    trace!("exiting event_loop");
}
