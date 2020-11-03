mod manipulate;
use manipulate::*;

#[no_mangle]
pub extern "C" fn do_thing() {
    hi();
}

#[no_mangle]
pub extern "C" fn fullscreen() {
    let hwnd = get_active_window();
    full_screen(hwnd);
    push_changes(hwnd);
}

#[no_mangle]
pub extern "C" fn opacity(alpha: i32) {
    let hwnd = get_active_window();
    enable_transparency(hwnd);
    set_opacity(hwnd, alpha as u8);
    push_changes(hwnd);
}
