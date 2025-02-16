// importação de biblioteca que entenda entrada e saida
use std::io;
fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

//cria uma função
fn main() {
    // Criação de uma string capaz de armazenar uma entrada
    let mut palavra = String::new();

    //print padrão da linguagem Rust
    println!("Digite uma palavra:");

    io::stdin()  
        .read_line(&mut palavra)  
        .expect("Erro ao ler a entrada");  

    //Deixa a palavra "Limpa"
    let palavra = palavra.trim();
    let invertida: String = reverse(palavra);
    
    //exibir palavra invertida
    println!("Palavra invertida: {}",invertida);
}

