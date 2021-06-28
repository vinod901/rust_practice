use std::io;
pub fn run() {
	let mut a = String::new();
	println!("Enter some string : ");
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
