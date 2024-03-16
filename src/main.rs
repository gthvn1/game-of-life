mod hello;

fn main() {
    let x: i32 = hello::get_number();
    println!("Got {x}!");
    hello::say_hello("Sailor".to_string());
}
