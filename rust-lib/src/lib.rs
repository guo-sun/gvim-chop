mod manipulate;
use manipulate::*;

#[no_mangle]
pub extern "C" fn do_thing() {
    hi();
}

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

#[no_mangle]
pub extern "C" fn position_window() {
}
