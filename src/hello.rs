use std::{
    ffi::{c_void, CString},
    os::raw::{c_char, c_int},
};

extern "C" {
    fn getNumber() -> c_int;
    fn sayHello(name: *const c_char) -> c_void;
}

pub fn get_number() -> i32 {
    unsafe { getNumber() }
}

pub fn say_hello(name: String) {
    let c_name = CString::new(name).unwrap();
    unsafe { sayHello(c_name.as_ptr() as *const c_char) };
}
