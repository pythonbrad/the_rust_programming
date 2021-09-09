fn main() {
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	println!("The value of x is: {}", x);
	let guess = "42".parse().expect("Not a number!");
	println!("Guess is {}", guess);
}
