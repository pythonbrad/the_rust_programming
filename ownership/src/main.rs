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

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
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
    println!("first word: {}", s);
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
    
    let mut s = String::from("hello world");
    
    let word = first_word(&s); // word will get the value 5
    
    println!("{}", word);
    
    s.clear(); // this empties the String, making it equal to ""
    
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value !( with. word is now totally invalid!
    
    no_dangle();
    
    let s = String::from("hello");
    let slice = &[0..2];
    let slice = &s[..2];
    let slice = &s[..];
    
    let my_string = String::from("hello world");
    
    // first_word works on slices of string literals
    let word = first_word(&my_string[..]);
    
    let my_string_literal = "hello world";
    
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
	// special happens.
	// Here, s3 goes out the scope and is dropped. s2 goes out of scope but was
	// moved, so nothing happens. s1 goes out of scope and is dropped.
