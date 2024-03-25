pub mod register {
    use std::io::{self, Write};

    #[derive(Debug)]
    pub struct People {
        pub name: String,
        pub birth_date: u8,
        pub sex: char,
        pub weight: f32,
        pub height: f32,
        pub nationality: String,
        pub career: String,
    }

    impl People {

        // recebe entrada do usuário e as insere numa instância de People
        pub fn register() -> People {

            let name: String = read_input("Name: ");
            let birth_date: u8 = match read_input("birth date: ").parse() {
                Ok(num) if num > 1 && num < 121 => num,
                _ => {
                    println!("Failed to parse birth date, it's an invalid birth date!");
                    0
                }
            };
    
            let sex: char = read_input("Sex: ").parse().expect("Invalid char!");
            let weight: f32 = match read_input("Weight: ").parse() {
                Ok(num) if num > 2.0 && num < 550.0 => num,
                _ => {
                    println!("Failed do parse weight, it's an invalid weight!");
                    0.0
                }
            };
    
            let height: f32 = match read_input("Height: ").parse() {
                Ok(num) if num > 20.0 && num < 300.0 => num,
                _ => {
                    println!("Failed to parse height, it's an invalid height!");
                    0.0
                },
            };
    
            let nationality: String = read_input("Nationality: ");
            let career: String = read_input("Career: ");
    
            People { name, birth_date, sex, weight, height, nationality, career }
    
        }
    }

    // recebe a entrada do usuário e a retorna em String
    fn read_input(prompt: &str) -> String {
        let mut input = String::new();
        
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout!");
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        input.trim().to_string()
    }
}
