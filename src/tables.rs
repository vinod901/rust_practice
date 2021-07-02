use std::io;
pub fn run() {
    let mut num = String::new();
    println!("Give some input :");
    io::stdin().read_line(&mut num).unwrap();
    let trimmed = num.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            for digit in 1..11 {
                println!("{} * {} = {}", i, digit, i * digit);
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    }
}
