use std::os::raw::c_int;

extern "C" {
    fn get_number() -> c_int;
}

fn main() {
    let x: i32 = unsafe { get_number() };
    println!("Got {x}!");
}
