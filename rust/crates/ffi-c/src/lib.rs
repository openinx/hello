#[no_mangle]
pub extern fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[warn(dead_code)]
fn main() {}