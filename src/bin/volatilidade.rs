//! Implementa exemplos do capítulo 15 do livro  
//! Hull, J. C. (2016). Options, futures, and other derivatives (11e).


use hull_rust::volatilidade; 
fn main() {
    volatilidade::ex154();

    let valor = volatilidade::volatilidade_ano(6.04, 126);
    println!("{}", valor);
 
}