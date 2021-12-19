use std::{env, fs};
#[allow(dead_code)]
pub fn present_dir() {
    println!("\nCurrent directory : {}", module_path!());
    println!("\nContents of {:?}\n", env::current_dir().unwrap());
    for file in fs::read_dir("./src").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
