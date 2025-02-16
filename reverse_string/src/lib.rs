pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// criação de uma função que inverte uma string
//Inverter a palavra 
    //".chars()"transforma a palavra em um iterator
    //".rev()"inverte a ordem das letras
    //".collect()"coleta as letras invertidas e tranforma em uma rstring
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
