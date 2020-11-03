use crate::manipulate::message::print;

use std::{
    mem,
    io::Error,
    ptr::null_mut,
    convert::TryInto
};

use winapi::{
    shared::{
        minwindef::{
            BOOL,
        },
        windef::{
            HWND,
            RECT,
        }
    },
    um::winuser::{
        GetActiveWindow,
        SetWindowLongW,
        GetWindowLongW,
        GetWindowRect,
        SetWindowPos,

        GetMonitorInfoW,
        MonitorFromWindow,

        MONITORINFO,

        SWP_NOMOVE,
        SWP_NOZORDER,
        SWP_NOSIZE,
        SWP_NOACTIVATE,

        MONITOR_DEFAULTTONEAREST,
    }
};

pub fn add_flags(source: i32, flags: i32) -> i32 {
    source | flags
}

pub fn remove_flags(source: i32, flags: i32) -> i32 {
    source & (!flags)
}

pub fn attempt(x: BOOL, msg: &str) -> BOOL {
    if x == 0 {
        print(&format!("Whoops! Couldn't {}.\n\n{}", msg, Error::last_os_error()));
    }
    x
}

pub fn remove_style(hwnd: HWND, style_flag: i32, style: u32) -> BOOL {
    attempt(unsafe {
        SetWindowLongW(
                hwnd,
                style_flag,
                remove_flags(
                    GetWindowLongW(hwnd, style_flag),
                    style as i32)
                )
        }, "add window style")
}

pub fn add_style(hwnd: HWND, style_flag: i32, style: u32) -> BOOL {
    attempt(unsafe {
        SetWindowLongW(
                hwnd,
                style_flag,
                add_flags(
                    GetWindowLongW(hwnd, style_flag),
                    style as i32)
                )
        }, "add window style")
}

pub fn get_style(hwnd: HWND, style_flag: i32) -> i32 {
    unsafe { GetWindowLongW(hwnd, style_flag) }
}

pub fn set_window_rect(hwnd: HWND,
                rect: RECT,
                flags: u32) -> BOOL {
    attempt(unsafe {
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

    if attempt(unsafe {
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

        if attempt(GetMonitorInfoW(
                monitor,
                &mut monitor_info)
          , "get monitor info") == 0 {
            None
        } else {
            Some(monitor_info.rcMonitor)
        }
    }
}

/// Calls SetWindowPos without changes to force redraw
pub fn push_changes(hwnd: HWND) -> BOOL {
    attempt(unsafe {
        SetWindowPos(
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
    unsafe { GetActiveWindow() }
}
