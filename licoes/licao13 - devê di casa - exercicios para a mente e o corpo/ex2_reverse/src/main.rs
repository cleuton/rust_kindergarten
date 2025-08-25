use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let texto = &args[1];
    let texto_reverso: String = texto.chars().rev().collect();
    println!("Texto reverso: {}", texto_reverso);
}
