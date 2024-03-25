use std::io;
use rand::Rng;

fn main() {
    println!("Bem vindo ao gerador de emojis aleatórios!");

	println!("Digite 'INICIAR' para começar!");
	
   	let mut input = String::new();
    
  	io::stdin().read_line(&mut input)
   		.expect("Erro ao tentar ler a linha!");

    loop {
    
    	let random_number = rand::thread_rng().gen_range(0,50);
    	let emoticon: char = char::from_u32(0x1F600 + random_number).unwrap();
    
    	if input.trim().eq_ignore_ascii_case("INICIAR") {
    		println!("E o emoticon aleatório do momento é: {}", emoticon);

    		println!("Deseja exeutar o programa novamente? (S/N)");

			let mut response = String::new();

    		io::stdin().read_line(&mut response)
    			.expect("Erro ao tentar ler linha!");

    		if response.trim().eq_ignore_ascii_case("S") {
    			continue;
    		} else {break;}
    		
    	} else {
    		println!("Digite 'INICIAR'!!");
    		continue;
    	}
    }
}
