pub fn run() {
    let mut a = String::new();
    println!("Give some input :");
    io::stdin()
        .read_line(&mut a)
        .expect("some error occured...!");
    println!("input : {}", a);
    let trimmed = a.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    }
}
