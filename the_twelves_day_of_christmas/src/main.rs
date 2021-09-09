fn main() {
	// data
    let lyrics = [
		"Two turtle doves, and",
		"Three french hens",
		"Four calling birds",
		"Five golden rings",
		"Six geese a-laying",
		"Seven swans a-swimming",
		"Eight maids a-milking",
		"Nine ladies dancing",
		"Ten lords a-leaping",
		"Eleven pipers piping",
		"Twelve drummers drumming",
	];
	let ranks = [
		"first",
		"second",
		"third",
		"fourth",
		"fifth",
		"sixth",
		"seventh",
		"eigth",
		"ninth",
		"tenth",
		"eleventh",
		"twelfth"
	];
	
	println!("----The Twelve Days of Christmas----\n\n");
	
	for i in 0..12 {
		println!("On the {} day of Christmas, my true love sent to me", ranks[i]);
		for ii in 0..i {
			println!("{}", lyrics[ii]);
		}
		println!("A partridge in a pear tree\n");
	}
}
