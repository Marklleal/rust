fn main() {
    let days_array = days();

    let phrases_array = phrases(); 

	print_music(days_array, phrases_array);	    
}

fn days() -> [&'static str; 12] {
	["first", "second", "third", "fourth", "fifth", "sixth", "seventh", 
    	"eighth", "ninth", "tenth", "eleventh", "twelfth"]
}

fn phrases() -> [&'static str; 12] {
	["And a Partridge in a pear tree.", "Two turtle doves", "Three French Hens,", 
	    "Four calling birds,", "Five golden rings,", "Six geese a laying,", "Seven swans a swimming,", 
	    "Eight maids a milking,", "Nine ladies dancing,", "Ten lords a leaping,", "Eleven pipers piping,", 
	    "Twelve drummers drumming,"]
}

fn print_music(days_array: [&'static str; 12], phrases_array: [&'static str; 12]) {
	for (elements, day) in days_array.iter().enumerate() {
		println!("On the {} day of Christmas", day);

		for index in (0..=elements).rev() {
			println!("{}", phrases_array[index]);
		}

		println!("");
	}	
}
