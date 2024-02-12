// Fibonacci generator

fn main() {
	fibonacci();
}

fn fibonacci() { 
	let mut number: u64 = 0;
	let mut fibonacci: u64 = 1;

	println!("{}", number);

	while fibonacci < 10000000000 {
		println!("{}", fibonacci);

		let next_number = number + fibonacci;
		number = fibonacci;
		fibonacci = next_number;
	}
}
