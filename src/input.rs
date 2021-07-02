use std::io;
pub fn run() {
	// creating a vairable lenght string variable
	let mut a = String::new();
	println!("Enter some string : ");

	// reading the input from the terminal and handling possible errors
	match io::stdin().read_line(&mut a) {
		Ok(_) => {
			println!("Success!");
			println!("input : {}", a);
		}
		Err(e) => {
			println!("Error occured : {}", e);
		}
	}
}
