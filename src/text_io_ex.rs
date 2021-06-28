#[macro_use]
extern crate text_io;
pub fn run() {
    // read until a whitespace and try to convert what was read into an i32
    let i: i32 = read!();
    println!("Read in: {}", i);
}
