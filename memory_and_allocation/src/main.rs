fn takes_ownership(some_string: String) { // some_string comes into scope
	println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
	println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
	let some_string = String::from("Hello");
	some_string
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
														// scope
	a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
	s.len() // len() returns the length of a String
} // Here, s goes out of the scope. But because it does not have ownership of what
	// it refers to, nothing happens.

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

fn main() {
	let s1 = String::from("hello");
	
	let s2 = calculate_length(&s1);
	
	println!("The lengt of '{}' is {}.", s1, s2);
	
    let s = String::from("hello");	// s comes into  scope
    takes_ownership(s);				// s's values moves into the function...
									// ... and so is no longer valid here
    let x = 5;						// x comes into scope
    
    makes_copy(x);					// x would move into the function,
									// but i32 is Copy, so it's okay to still
									// use x afterward
    
    let s1 = gives_ownership();
    
    let s2 = String::from("hello");
    
    let s3 = takes_and_gives_back(s2);
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
	// special happens.
	// Here, s3 goes out the scope and is dropped. s2 goes out of scope but was
	// moved, so nothing happens. s1 goes out of scope and is dropped.
