use std::os::raw::c_int;

#[link(name = "example")]
unsafe extern "C" {
    fn add_numbers(a: c_int, b: c_int) -> c_int;
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { add_numbers(a as c_int, b as c_int) }
}
