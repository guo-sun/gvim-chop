mod manipulate;
use manipulate::*;

#[no_mangle]
pub extern "C" fn fullscreen() {
    with_active_gvim(
        |hwnd| { full_screen(hwnd); });
}

#[no_mangle]
pub extern "C" fn opacity(alpha: i32) {
    with_active_gvim(
        |hwnd| {
            enable_transparency(hwnd);
            set_opacity(hwnd, alpha as u8);
        });
}

#[no_mangle]
pub extern "C" fn remove_title_bar() {
    with_active_gvim(
        |hwnd| { remove_title(hwnd); });
}

use std::os::raw::c_char;
use std::ffi::CStr;

/// Takes string percent and returns an i32 between low and high
fn str_perc_to_pos(percent: &str, low: i32, high: i32) -> i32 {
    let f: f32 = percent.parse().unwrap();

    let bottom = (f * (high as f32) / 100.0).ceil() as i32;

    bottom + low
}

#[no_mangle]
pub extern "C" fn position_window(pos_str: *const c_char) -> i32 {
    let pos = unsafe {
        let s = CStr::from_ptr(pos_str);
        s.to_str().unwrap().to_string()
    };

    let s: Vec<&str> = pos.split("-").collect();

    match s[..] {
        [a, b, c, d] => {
            with_active_gvim(
                |hwnd| {
                    if let Some(monitor_rect) = flags::get_monitor_rect(hwnd) {
                        let x = str_perc_to_pos(a, monitor_rect.left, monitor_rect.right);
                        let y = str_perc_to_pos(b, monitor_rect.top, monitor_rect.bottom);
                        let w = str_perc_to_pos(c, monitor_rect.left, monitor_rect.right);
                        let h = str_perc_to_pos(d, monitor_rect.top, monitor_rect.bottom);

                        flags::set_window_pos(hwnd, x, y, w, h, 0);
                    }
                });

            1
        },
        _ => {
            message::print(
                &"Position string looks wrong. Should be 'x-y-w-h'.");
            0
        }
    }
}

#[no_mangle]
/// 0 false, 1 true
pub extern "C" fn pin_window(do_pin: i32) {
    with_active_gvim(
        |hwnd| {
            flags::set_window_istopmost(
                hwnd, 
                if do_pin == 0 {
                    false
                } else {
                    true
                });
        });
}
