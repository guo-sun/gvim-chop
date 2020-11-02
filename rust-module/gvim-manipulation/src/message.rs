use std::io::Error;
use std::ptr::null_mut;

use winapi::um::winuser::{
    MB_OK,
    MessageBoxW,
};

pub fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();

    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };

    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}

pub fn print(msg: &str) {
    if let Err(e) = print_message(&msg) {
        println!("Error: {}", e);
    }
}

