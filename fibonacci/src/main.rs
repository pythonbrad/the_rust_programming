use std::io;

// fib(n) = fib(n-1) + fib(n-2)
fn fib(n: u32) -> u32 {
	if n == 1 || n == 2 {
		1
	} else {
		fib(n-2) + fib(n-1)
	}
}

fn main() {
	let mut value = String::new();
	
	println!("Fibonnaci");
	
	// We get the rank
	println!("Enter the rank: ");
	io::stdin()
		.read_line(&mut value)
		.expect("Failed to read");
	let mut value: u32 = value
		.trim()
		.parse()
		.expect("An positive integer is expected");
	
	// We get the value at nth rank
	value = fib(value);
	
	println!("The fib at this rank is {}", value);
}
