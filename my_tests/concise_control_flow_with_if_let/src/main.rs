mod us_states;  // importa um arquivo que contém todos os estados dos EUA
use std::io;
use rand::seq::SliceRandom;
use rand::Rng;
use us_states::get_us_states;

#[derive(Debug)]
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

impl Coins {
    // gera um número aleatório para respectivamente fazer a correspondência do valor da moeda
    fn random(state: &str) -> Self {
        let index = rand::thread_rng().gen_range(0..=3);

        match index {
            0 => Coins::Penny,
            1 => Coins::Nickel,
            2 => Coins::Dime,
            _ => Coins::Quarter(state.to_string()),
        }
    }

    // faz a correspondência da moeda com o seu(s) valor(es)
    fn match_coins(coins: Coins, state: &str) -> u8 { 
        match coins {
            Coins::Penny => 1,
            Coins::Nickel => 5,
            Coins::Dime => 10,
            Coins::Quarter(_) => { 
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // exibe o valor e o nome de cada moeda que aparece no loop
    fn print_coin(nmb: u8) {
        match nmb {
            1 => println!("Coin: Penny\nValue: 1"),
            5 => println!("Coin: Nickel\nValue: 5"),
            10 => println!("Coin: Dime\nValue: 10"),
            _ => (),
        }
    }
}

/*
A função `random_state` retorna um estado aleatório dos Estados Unidos.

 # Parâmetros

 * `states`: Uma referência para um vetor de referências de strings. Cada string é o nome de um estado dos EUA.

 # Retorno

 A função retorna uma `Option` que pode conter uma referência para uma string. Se o vetor `states` estiver vazio, a função retornará `None`.

 # Exemplos

 ```
 let states = vec!["California", "Texas", "New York"];
 let state = random_state(&states);
 println!("{:?}", state);
 ```
*/
fn random_state<'a>(states: &'a Vec<&'a str>) -> Option<&'a str> {
    // Cria um gerador de números aleatórios.
    let mut rng = rand::thread_rng();
    
    // Escolhe um elemento aleatório do vetor `states`.
    let choose = states.choose(&mut rng);
    
    // Retorna uma cópia da referência contida na `Option` retornada por `choose`.
    // Se `choose` retornou `None`, `copied` também retornará `None`.
    choose.copied()
}

fn looping() {
    let states = get_us_states(); // recebe um vetor com os estados dos EUA
    let mut count = 0; // contador
    
    println!("welcome!\nInitializing...");

    loop {
        let mut choice = String::new(); // escolha da continuação do loop

        // sorteia um estado americano 
        if let Some(state) = random_state(&states) {
            let coin = Coins::random(state);

            // verifica se a moeda vai ser um quarter ou não
            if let Coins::Quarter(state) = coin {
                println!("State quarter from {:?}!", &state);
                println!("Coin: Quarter\nValue: 25");
            } else {
                let coin_value = Coins::match_coins(coin, state);
                Coins::print_coin(coin_value);
                count += 1
            }
        } else { // caso o vetor com o nome dos estados esteja vazio
            println!("The Vector with the name of the states is empty!");
        };

        println!("Run again? (y/s)");
        io::stdin().read_line(&mut choice).expect("Failed to read line!");

        // faz um parse de String pra char
        if let Some(choice) = choice.trim().to_lowercase().as_str().chars().next() { 
            // verifica a escolha feita
            if !(choice == 'y' || choice == 's') {
                println!("The number of times a currency other than 'non-quarter' appeared is: {count}");
                break;
            }
        }
    }
}

fn main() {
    looping();
}
