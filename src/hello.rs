use std::os::raw::c_int;

extern "C" {
    fn getNumber() -> c_int;
}

pub fn get_number() -> i32 {
    return unsafe { getNumber() };
}
