use std::io;
pub fn run() {
    // creating a variable length string variable
    let mut a = String::new();

    // taking the user input as a string
    println!("Give some input :");
    io::stdin().read_line(&mut a).unwrap();
    println!("input : {}", a);

    // converting a string input into u32 by using the parser
    let trimmed = a.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer is {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    }
}
