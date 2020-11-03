pub mod flags;
mod message;

use flags::*;
use message::*;

use std::io::Error;
use std::convert::TryInto;
use std::ptr::null_mut;
use std::mem;

use winapi::shared::minwindef::{
    BOOL,
};

use winapi::shared::windef::{
    HWND,
    RECT,
};

use winapi::um::winuser::{
    GetActiveWindow,
    SetWindowLongW,
    GetWindowLongW,
    SetLayeredWindowAttributes,
    GetWindowRect,
    SetWindowPos,

    GetMonitorInfoW,
    MonitorFromWindow,

    MONITORINFO,

    GWL_EXSTYLE,
    GWL_STYLE,

    SWP_NOMOVE,
    SWP_NOZORDER,
    SWP_NOSIZE,
    SWP_NOACTIVATE,

    LWA_ALPHA,

    MONITOR_DEFAULTTONEAREST,

    WS_THICKFRAME,
    WS_CAPTION,
    WS_EX_LAYERED,
};

pub fn t(x: BOOL, msg: &str) -> BOOL {
    if x == 0 {
        print(&format!("Whoops.. Couldn't {}\n\n{}", msg, Error::last_os_error()));
    }
    x
}

pub fn remove_style(hwnd: HWND, style_flag: i32, style: u32) -> BOOL {
    unsafe {
        t(SetWindowLongW(
                hwnd,
                style_flag,
                flags_remove(
                    GetWindowLongW(hwnd, style_flag),
                    style as i32)
                )
        , "add window style")
    }
}

pub fn add_style(hwnd: HWND, style_flag: i32, style: u32) -> BOOL {
    unsafe {
        t(SetWindowLongW(
                hwnd,
                style_flag,
                flags_add(
                    GetWindowLongW(hwnd, style_flag),
                    style as i32)
                )
        , "add window style")
    }
}

pub fn get_style(hwnd: HWND, style_flag: i32) -> i32 {
    unsafe { GetWindowLongW(hwnd, style_flag) }
}

pub fn remove_title(hwnd: HWND) -> BOOL {
    t(remove_style(
            hwnd,
            GWL_STYLE,
            WS_CAPTION)
            , "set window style")
}

pub fn enable_transparency(hwnd: HWND) -> BOOL {
    t(add_style(
            hwnd,
            GWL_EXSTYLE,
            WS_EX_LAYERED)
            , "add transparency")
}

pub fn full_screen(hwnd: HWND) -> BOOL {
    if let Some(rect) = get_monitor_rect(hwnd) {
        remove_style(hwnd, GWL_STYLE, WS_THICKFRAME);
        set_window_rect(hwnd, rect, 0)
    } else {
        0
    }
}

pub fn set_opacity(hwnd: HWND, opacity: u8) -> BOOL {
    t(unsafe{SetLayeredWindowAttributes(
        hwnd,
        0,
        opacity,
        LWA_ALPHA
    )}, "set opacity")
}

pub fn set_window_rect(hwnd: HWND,
                rect: RECT,
                flags: u32) -> BOOL {
    t(unsafe {
        SetWindowPos(
            hwnd,
            null_mut(),
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
            flags)
    }, "set position")
}

pub fn get_window_rect(hwnd: HWND) -> Option<RECT>{
    let mut rect : RECT = unsafe { mem::zeroed() };

    if t(unsafe {
        GetWindowRect(hwnd, &mut rect)
    }, "get window rect") == 0 {
        None
    } else {
        Some(rect)
    }
}

pub fn get_monitor_rect(hwnd: HWND) -> Option<RECT> {
    unsafe {
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

        let mut monitor_info : MONITORINFO = mem::zeroed();
        monitor_info.cbSize = mem::size_of::<MONITORINFO>().try_into().unwrap();

        if t(GetMonitorInfoW(
                monitor,
                &mut monitor_info)
          , "get monitor info") == 0 {
            None
        } else {
            Some(monitor_info.rcMonitor)
        }
    }
}

/// Calls SetWindowPos without changing to force redraw
pub fn push_changes(hwnd: HWND) -> BOOL {
    t(unsafe{SetWindowPos(
        hwnd,
        null_mut(),
        0,
        0,
        0,
        0,
        SWP_NOACTIVATE | SWP_NOZORDER | SWP_NOSIZE | SWP_NOMOVE
    )}, "set window pos")
}

pub fn get_active_window() -> HWND {
    unsafe{GetActiveWindow()}
}

pub fn hi() {
    let hwnd = get_active_window();

    if let Some(rect) = get_window_rect(hwnd) {
        print(&format!("Window is:\n{} {} {} {}"
                       , rect.top
                       , rect.bottom
                       , rect.left
                       , rect.right));
    }

    if let Some(rect) = get_monitor_rect(hwnd) {
        print(&format!("Monitor is:\n{} {} {} {}"
                       , rect.top
                       , rect.bottom
                       , rect.left
                       , rect.right));
    }

    remove_title(hwnd);
    enable_transparency(hwnd);
    set_opacity(hwnd, 240u8);
    full_screen(hwnd);
    push_changes(hwnd);
}
