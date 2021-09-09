use std::io;

fn main() {
	// We init variable
	let mut from_unit = String::new();
	let mut to_unit = String::new();
	let mut value = String::new();
	
	println!("Temperature Converter");
	
	// We get the source unit
	println!("Enter the source unit: ");
	io::stdin()
		.read_line(&mut from_unit)
		.expect("Failed to read");
	let from_unit:char = from_unit
		.trim()
		.parse()
		.expect("A character was expected");
	
	// We get his value
	println!("Enter the value to convert: ");
	io::stdin()
		.read_line(&mut value)
		.expect("Failed to read");
	let mut value:f32 = value
		.trim()
		.parse()
		.expect("A decimal value was expected");
	
	// We get the dest unit
	println!("Enter the dest unit: ");
	io::stdin()
		.read_line(&mut to_unit)
		.expect("Failed to read");
	let to_unit: char = to_unit
		.trim()
		.parse()
		.expect("A charecter was expected");
	
	// We perform the convertion
	value = {
		if from_unit == to_unit {
			value
		} else if (from_unit, to_unit) == ('c', 'f') {
			value * 9.0 / 5.0 + 32.0
		} else if (from_unit, to_unit) == ('f', 'c') {
			(value - 32.0) / 9.0 * 5.0
		} else {
			println!("The units combination is not taken in charge");
			0.0
		}
	};
	
	println!("The value after conversion is: {}", value);
}
