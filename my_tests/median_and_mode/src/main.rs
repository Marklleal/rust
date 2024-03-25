/*
    Objetivos do programa:
    O programa deve receber uma lista númerica de inteiros
    e retornar duas informações:
    Mediana e Moda
    A mediana é o valor que fica no meio da lista quando ela está ordenada. 
    A moda é o valor que aparece com mais frequência na lista. 
    Então, antes de tudo, preciso ordenar a lista, 
    posso usar apenas um vetor (foi usado mais de um vetor pois dividi
    o programa em quatro funções)
*/

use std::{collections::HashMap, ops::RangeInclusive};
use rand::Rng;

/*  rand_int_gen(.., u32)

    responsável por gerar um número inteiro aleatório, 
    de acordo com o range passado, e a quantidade de 
    números que devem ser gerados

    a função retorna um vetor com todos os números 
    aleatórios que foram gerados
*/
fn rand_int_gen(def_range: &RangeInclusive<i32>, nmb_times: u32) -> Vec<i32> {

    let mut range = rand::thread_rng();
    let mut vec_nmbrs: Vec<i32> = Vec::new(); // vetor de retorno
    let mut size: u32 = 1;
    
    while size <= nmb_times {

        let def_range = *def_range.start()..=*def_range.end();
        let num = range.gen_range(def_range);

        vec_nmbrs.push(num);
        size += 1;

    }

    vec_nmbrs
}

/*  list_median(&[i32])

    recebe como argumento de um parâmetro, um 
    slice do tipo i32 referido como imutável;

    após isso, um bloco de caso faz o cálculo 
    da mediana baseando-se em casos do número 
    ser ímpar ou par.

    por fim, é retornado um valor de ponto 
    fluante do tipo f64

*/
fn list_median(vec_nmbrs: &[i32]) -> f64 {

    let mut sorted_list: Vec<i32> = vec_nmbrs.to_owned();
    sorted_list.sort(); // organiza a lista
    let list_len = sorted_list.len() as f64; // recebe o tamanho da lista e o converte

    let median: f64 = if list_len % 2.0 != 0.0 { // caso a lista seja ímpar

        let num = (list_len / 2.0 - 0.5) as usize;
        sorted_list[num] as f64

    } else { // caso a lista seja par

        let num = (list_len / 2.0 - 1.0) as usize;
        ((sorted_list[num] + sorted_list[num + 1]) as f64) / 2.0

    };
    
    median // variável de retorno
}

/*  mode(&[i32])

    faz o cálculo da moda de uma lista de números inteiros.
    devolve como retorno um vetor do tipo i32

*/
fn mode(vec_nmbrs: &[i32]) -> Vec<i32> {

    let mut map = HashMap::new();
    let mut vec_mode_val: Vec<i32> = Vec::new(); // valor da moda

    for i in vec_nmbrs { // calcula quantas vezes cada valor apareceu
        *map.entry(i).or_insert(0) += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    for (&k, &v) in &map { // percorre os pares de acordo com as keys 

        if v == max_value { // itera sobre o vetor, caso o value seja igual o max_value 
            vec_mode_val.push(*k);
        }
    }

    vec_mode_val // variável de retorno
}

fn main() {
    let mut vec_nmbrs = rand_int_gen(&(0..=10), 5);
    let mut mode = mode(&vec_nmbrs);

    println!("Integer list: {:?}", {
        vec_nmbrs.sort();
        &vec_nmbrs
    });
    println!("The median is: {:?}", list_median(&vec_nmbrs));
    println!("The mode is: {:?}", {
        mode.sort();
        &mode
    });
}
