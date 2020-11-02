pub fn flags_add(source: i32, flags: i32) -> i32 {
    source | flags
}

pub fn flags_remove(source: i32, flags: i32) -> i32 {
    source & (!flags)
}
