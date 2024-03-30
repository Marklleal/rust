// Jogo da forca, CLI

mod words;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io;

/*
    Recebe como parâmetro um vetor de &str e retorna
    um Option de uma &str específica de forma aleatória
*/
fn random_word<'a>(words: &'a Vec<&'a str>) -> Option<&'a str> {
    let mut range = rand::thread_rng();
    let choose = words.choose(&mut range);
    choose.copied()
}

// Retorna o caractere que foi digitado pelo usuário
fn user_input_char() -> char {
    let mut input = String::new();

    println!("Digite apenas uma letra: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler linha!");

    // converte a string para char
    let input_char: char = match input.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("É preciso ser inserido apenas um caractere!");
            std::process::exit(1);
        }
    };
    input_char
}

/*
    Dá ao usuário opções de exibição após a 
    execução ocorrer corretamente;
    Retorna um valor u8, que designa a opção 
    de exibição escolhida
*/
fn user_input_final() -> u8 {
    let mut input = String::new();

    println!(
        "Escolha uma das opções de exibição:\n
        1- Palavra sorteada\n
        2- Número de tentativas\n
        3- Letras inseridas\n
        4- Etapas de descoberta\n
        5- Todas acima\n
        6- Sair"
    );

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler linha!");

    let choice: u8 = match input.trim().parse() {
        Ok(n) if n > 0 && n < 7 => n,
        _ => {
            println!("Nenhum número de 1 a 6 foi escolhido!");
            0
        }
    };
    choice
}

/*
    Quantidade de tentativas que ainda estão disponíveis,
    sempre que a função é chamada, ela decrementa o valor
    natural até que chegue em zero, caso ocorra;
    Retorna um valor u8 natural para que o valor original
    seja atualizado toda vez que a função for executada;
*/
fn remaining_attempts(mut rem_att: u8, word: &str) -> u8 {
    rem_att -= 1;

    if rem_att == 0 {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap(); // limpa o terminal
    
        println!("Fim de jogo!!\nA palavra era {word}.\nJogue novamente :)");
        std::process::exit(1);
    } 
    println!("Tentativas restantes: {rem_att}");
    rem_att
}

/*
    Recebe como argumento no parâmetro um &str,
    que designa a palavra sorteada;
    O principal funcionamento do código está nessa 
    função.
*/
fn execute_all(word: &str) {
    let mut attempts: Vec<String> = Vec::new();
    let mut current_attempt = String::new();
    let mut tot_attempts: u32 = 0;
    let mut char_attempts: Vec<char> = Vec::new();
    let mut rem_att: u8 = 9;

    // current_attempt é preenchido com '_' de acordo com o tamanho de 'word'
    for _ in 0..word.len() {
        current_attempt.push('_');
    }

    loop {
        println!("Tentativa atual: {}", current_attempt);

        let input_char = user_input_char();
        let mut new_attempt = String::new();

        for (index, value) in word.char_indices() {
            // caso a entrada do usuário seja igual ao valor do índice atual, 
            // ou caso o caractere do índice da tentativa atual seja diferente de '_':
            if value == input_char || current_attempt.chars().nth(index).unwrap() != '_' {
                new_attempt.push(value);
            } else {
                new_attempt.push('_');
            }
        }

        if new_attempt == current_attempt { // caso no momento o usuário não tenha acertado uma letra
            rem_att = remaining_attempts(rem_att, word);
        } else { // caso tenha acertado alguma letra da palavra
            attempts.push(new_attempt.clone());
            current_attempt = new_attempt;
        }

        if !current_attempt.contains('_') {
            execute!(io::stdout(), Clear(ClearType::All)).unwrap(); // limpa o terminal
            println!("Parabéns! Você adivinhou a palavra!");
            
            let choice = user_input_final();
            
            match choice {
                1 => println!("Palavra sorteada: {word}"),
                2 => println!("Total de tentativas: {tot_attempts}"),
                3 => println!("Letras digitadas: {:?}", char_attempts),
                4 => println!("Etapas de descoberta: {:?}", attempts),
                5 => {
                    println!("Palavra sorteada: {word}");
                    println!("Total de tentativas: {tot_attempts}");
                    println!("Letras digitadas: {:?}", char_attempts);
                    println!("Etapas de descoberta: {:?}", attempts);
                }
                6 => {
                    println!("Saindo...");
                    std::process::exit(1);
                }
                _ => {
                    println!("Você deveria ter escolhido um número de 1 a 6!");
                    std::process::exit(1);
                }
            }
            break;
        } 
        tot_attempts += 1;
        char_attempts.push(input_char);
    }
}

fn main() {
    let words = words::get_words();

    match random_word(&words) {
        Some(word) => {
            execute_all(word);
        }
        None => {
            println!("Houve um erro imprevisto no funcionamento do código, algo foi alterado!");
        }
    }
}
