use std::io;
use std::cmp::Ordering;

fn compute(number: usize, index: usize, mut data: [usize; 256]) -> usize {
	// We verify if the current combination is correct
	let mut sum = 0;
	let mut nb_comb = 0;
	
	// We calcul the sum
	for i in data[0..=number].iter() {
		sum = sum + i;
	}
	match sum.cmp(&number) {
		Ordering::Less => {
			// We build a combination
			for value in 0..=number {
				// We save the current value
				data[index] = value;
				// We verify if the index is ok and we go to the next index
				if index < number {
					nb_comb += compute(number, index+1, data);
				} else {
					break;
				}			
			}
			nb_comb
		},
		Ordering::Greater => 0,
		Ordering::Equal => {
			// We show the current combination
			for i in 0..number {
				//print!(" {}", data[i]);
				if i + 1 == number {
					//println!(" = {}", number);
				} else {
					//print!(" +");
				}
			}
			1
		},
	}
}

fn main() {
	// We declare data
    let mut number = String::new();
    let data: [usize; 256] = [0; 256];
    
    // We get a number from the user
    println!("Enter a number: ");
    io::stdin().read_line(&mut number).expect("Failed to read");
    let number:usize = number.trim().parse().expect("A digit is expected");
    
    // We compute the combination of sum
	println!("Computed in {} combinations", compute(number, 0, data));
}
