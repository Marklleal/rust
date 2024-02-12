use std::io;

fn main() {
	let mut _input = String::new();
	let mut value = String::new();
	let input: u8 = initialize();
    
	println!("Enter the value for the conversion: ");

	io::stdin().read_line(&mut value) 
		.expect("Failed to read line");
	let value: f64 = match value.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Enter an integer from 1 to 6!");
			return;
		}
	};

	let result = match input {
		1 => celsius_to_kelvin(value),
		2 => celsius_to_fahrenheith(value),
		3 => fahrenheith_to_celsius(value),
		4 => fahrenheith_to_kelvin(value),
		5 => kelvin_to_celsius(value),
		6 => kelvin_to_fahrenheith(value),
		_ => {
			println!("Invalid input! Please choose a valid option!");
			return;
		}
	};

	println!("The conversion of {}º is {:.2}º", value, result);
}

fn initialize() -> u8 {
	loop {
		let mut user_input = String::new();
		
		println!("What temperature conversion do you want to do?");
		println!("1 = celsius to kelvin");
		println!("2 = celsius to fahrenheith");
		println!("3 = fahrenheith to celsius");
		println!("4 = fahrenheith to kelvin");
		println!("5 = kelvin to celcius");
		println!("6 = kelvin to fahrenheith");	
	
		io::stdin().read_line(&mut user_input)
			.expect("Failed to read line!");

		match user_input.trim().parse() {
       	    Ok(num) => match num {
       	    	1..=6 => break num,
       	    	_ => println!("Enter an integer from 1 to 6!"),
       	    }
            _ => println!("Enter an integer from 1 to 6!"),
        }
    }
}

fn celsius_to_kelvin(x: f64) -> f64 {
	if x < -273.15 {
		println!("It's impossible for kelvin to be less than 0!!!");
		-1.0 // standard error value
	} else {
		x + 273.15	
	}
}

fn celsius_to_fahrenheith(x: f64) -> f64 {
	if x < -273.17 {
		println!("It's impossible for kelvin to be less than 0!!!");
		-1.0 // standard error value
	} else {
		(9.0 / 5.0) * x + 32.0
	}
}

fn fahrenheith_to_celsius(x: f64) -> f64 {
	if x < -459.67 {
        println!("It's impossible for kelvin to be less than 0!!!");
		-1.0 // standard error value
	} else {
		(x - 32.0) / 1.8
	}
}

fn fahrenheith_to_kelvin(x: f64) -> f64 {
	if x < -459.67 {
        println!("It's impossible for kelvin to be less than 0!!!");
        -1.0 // standard error value
	} else {
		5.0 / 9.0 * (x - 32.0) + 273.15	
	}
}

fn kelvin_to_celsius(x: f64) -> f64 {
    if x < 0.0 {
        println!("It's impossible for kelvin to be less than 0!!!");
        -1.0 // standard error value
    } else {
        x - 273.15
    }
}

fn kelvin_to_fahrenheith(x: f64) -> f64 {
	if x < 0.0 {
		println!("It's impossible for kelvin to be less than 0!!!");
		-1.0 // standard error value
	} else {
		9.0 / 5.0 * (x - 273.15) + 32.0	
	}
}
