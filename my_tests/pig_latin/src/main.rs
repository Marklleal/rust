/*
    Terei que fazer com que esse programa receba uma 
    string e converta as palvras contidas nessa string 
    para o 'pig latin', funcionará da seguinte forma:
     * A primeira consoante de cada palavra é movida para 
        o final da palavra e “ay” é adicionado. 
     * Palavras que começam com uma vogal têm “hay” 
        adicionado ao final. Tendo em mente os detalhes 
        sobre a codificação UTF-8!
*/

use std::io; // entrada do usuário

const VOGALS: [char; 5] = ['a', 'e', 'i', 'o', 'u']; 

/*  separate_string_into_words(&str) 

    ao chamar a função e adicionar uma string que não esteja 
    vazia, cada palavra da frase adicionada será separada e 
    inserida em um array de Strings;
    caso a string usada esteja vazia, será retornado uma 
    mensagem de pane, pois esta não pode se encontrar vazia!

*/
fn separate_string_into_words(s: &str) -> Vec<String> {
    if s.len() == 0 {
        panic!("The vector is empty!!");
    } 

    // vetor que será usado para a inserção das palavras:
    let mut words: Vec<String> = Vec::new(); 
    
    // a cada espaço branco encontrado, os caracteres são 
    // separados como uma nova palavra e então é inserido no vetor
    for i in s.split_whitespace() { 
        words.push(i.to_string())    
    }
    words
}

/*  return_ay_or_hay(separate_string_into_words(&str))

    recebe um array contendo palavras que foram anteriormente
    seperadas, e, então, verifica o primeiro caractere de cada
    palavra. Sendo vogal, será adicionado "hay" em seu final;
    Sendo consoante, o primeiro caractere será movido para o 
    final com o acréscimo de "ay".

 */
fn return_ay_or_hay(mut words: Vec<String>) -> Vec<String> {
    for word in &mut words {
        if let Some(first_char) = word.chars().next() { 
            if VOGALS.contains(&first_char) {
                word.push_str("hay");
            } else { // caso o primeiro caractere seja consoante
                word.insert(word.len(), first_char);
                word.remove(0);
                word.push_str("ay");
            }
        }
    }
    words
}


fn main() {
    let mut user_input = String::new();

    println!("Type a word/phrase here: ");
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line!");

    println!("{:?}", return_ay_or_hay(separate_string_into_words(&user_input.trim())));
}
