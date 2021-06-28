use std::io;
pub fn run() {
    let mut a = String::new();
    println!("Give some input :");
    io::stdin().read_line(&mut a).unwrap();
    println!("input : {}", a);
    let trimmed = a.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer is {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    }
}
