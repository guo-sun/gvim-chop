pub mod flags;
pub mod message;

use flags::*;

use winapi::{
    shared::{
        minwindef::{
            BOOL,
        },
        windef::{
            HWND,
        }
    },
    um::winuser::{
        SetLayeredWindowAttributes,

        GWL_EXSTYLE,
        GWL_STYLE,

        LWA_ALPHA,
        LWA_COLORKEY,

        WS_THICKFRAME,
        WS_EX_LAYERED,
        WS_CAPTION,
    }
};

pub fn add_title(hwnd: HWND) -> BOOL {
    attempt(add_style(
            hwnd,
            GWL_STYLE,
            WS_CAPTION)
            , "set window style")
}

pub fn remove_title(hwnd: HWND) -> BOOL {
    attempt(remove_style(
            hwnd,
            GWL_STYLE,
            WS_CAPTION)
            , "set window style")
}

pub fn disable_transparency(hwnd: HWND) -> BOOL {
    attempt(remove_style(
            hwnd,
            GWL_EXSTYLE,
            WS_EX_LAYERED)
            , "add transparency")
}

pub fn enable_transparency(hwnd: HWND) -> BOOL {
    attempt(add_style(
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

pub fn set_transparent_color(hwnd: HWND, crkey: u32) -> BOOL {
    attempt(unsafe { SetLayeredWindowAttributes(
        hwnd,
        crkey,
        0,
        LWA_COLORKEY
    )}, "set transparent color")
}

pub fn set_opacity(hwnd: HWND, opacity: u8) -> BOOL {
    attempt(unsafe { SetLayeredWindowAttributes(
        hwnd,
        0,
        opacity,
        LWA_ALPHA
    )}, "set opacity")
}

pub fn with_active_gvim(on_hwnd: impl Fn(HWND)) {
    let hwnd = get_active_window();
    on_hwnd(hwnd);
    push_changes(hwnd);
}
