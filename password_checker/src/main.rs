use std::io;

fn is_lower(c: char) -> bool {
	let lower_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
	for i in lower_letters {
		if i == c {
			return true;
		}
	}
	return false;
}

fn is_upper(c: char) -> bool {
	let upper_letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
	for i in upper_letters {
		if i == c {
			return true;
		}
	}
	return false;
}

fn is_punctuation(c: char) -> bool {
	let punctuations = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
	for i in punctuations {
		if i == c {
			return true;
		}
	}
	return false;
}

fn is_digit(c: char) -> bool {
	let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
	for i in digits {
		if i == c {
			return true;
		}
	}
	return false;
}

fn main() {
	// An password is valid if it contains at least 8 characters,
	// begin by a lower letter,
	// include a capital letter,
	// a number and a special character
	loop {
		let mut validity = (0, 0, 0);
		let mut password = String::new();
		println!("Enter an password:");
		io::stdin()
			.read_line(&mut password)
			.expect("Failed to read");
		// We remove the inutile space
		let password = password.trim();
		// We verify the password
		if password.len() >= 8 && is_lower(password.chars().nth(0).expect("password is empty")) {
			for c in password.chars() {
				if is_upper(c) {
					validity.0 = 1;
				} else if is_punctuation(c) {
					validity.1 = 1;
				} else if is_digit(c) {
					validity.2 = 1;
				} else {}
			}
			// We verify the validity
			if validity == (1, 1, 1) {
				break;
			} else {
				if validity.0 == 0 {
					println!("Should contains at least 1 upper letter");
				} else if validity.1 == 0 {
					println!("Should contains at least 1 punctuation");
				} else if validity.2 == 0 {
					println!("Should contains at least 1 digit");
				}
			}
		} else {
			println!("Should have at least 8 characters and begin with a lower letter");
		}
	}
}
