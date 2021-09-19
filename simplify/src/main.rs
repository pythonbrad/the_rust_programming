use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = 1;
    
    println!("Enter a:");
    io::stdin()
		.read_line(&mut a)
		.expect("Failed to read");
	let a: usize = a.trim().parse().expect("An positive integer is expected");
    println!("Enter b:");
    io::stdin()
		.read_line(&mut b)
		.expect("Failed to read");
	let b: usize = b.trim().parse().expect("An positive integer is expected");
	for i in 1..=(if a < b {a} else {b}) {
		if (a * b) % i == 0 {
			c = i;
		}
	}
	println!("The simplify form of {}/{} is {}/{}", a, b, a/c, b/c);
}
