use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Guess game");
	let secret_number = rand::thread_rng().gen_range(1..101);
	
	loop {
		println!("Guess number");
		let mut guess_number = String::new();
		io::stdin().read_line(&mut guess_number).expect("Failed to read");
		
		let guess_number: i32 = match guess_number.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		match guess_number.cmp(&secret_number) {
			Ordering::Less => println!("It's greater!"),
			Ordering::Greater => println!("It's less!"),
			Ordering::Equal => break,
		};
	};
	
	println!("You won!");
}
