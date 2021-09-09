use std::io;

fn main() {
    // addition
    let sum = 5 + 10;
    
    // subtraction
	let difference = 95.5 - 4.3;
	
	// multiplication
	let product = 4 * 30;
	
	// division
	let quotient = 56.7 / 32.2;
	
	// remainder
	let remainder = 43 % 5;
	
	// boolean
	let t = true;
	let f: bool = false; // with explicit type annotation
	
	// tuple
	let tup = (500, 6.4, 1);
	
	let (x, y, z) = tup;
	
	println!("The value of y is: {}", y);
	
	let x: (i32, f64, u8) = (500, 6.4, 1);
	
	let five_hundred = x.0;
	
	let six_point_four = x.1;
	
	let one = x.2;
	
	let months = ["Monday", "Tuesday", "Wednesday"];
	
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	
	let b = [3; 5];
	
	let first = a[0];
	let second = a[1];
	
	println!("Please enter an array index.");
	
	let mut index = String::new();
	
	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line");
	
	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number");
		
	let element = a[index];
	
	println!(
		"The value of the element at index {} is: {}",
		index, element
	);
	
	another_function(5, 6);
	
	let x = 5;
	
	let y = {
		let x = 3;
		x + 1
	};
	println!("The value of y is {}", y);
	
	let x = five();
	println!("The value of x is {}", x);
	
	let x = plus_one(x);
	println!("The value of x is {}", x);
	
	let lucky_number = 7; // I'mfeeling lucky today
}

fn another_function(x: i32, y: i32) {
	println!("Another function with argument x={} and y={}", x, y);
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x+1
}
