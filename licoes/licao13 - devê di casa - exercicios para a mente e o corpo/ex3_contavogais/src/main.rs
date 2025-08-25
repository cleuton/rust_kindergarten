use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let texto = &args[1];
    let mut contador = 0;
    for c in texto.chars() {
        if "aeiouAEIOU".contains(c) {
            contador += 1;
        }
    }
    println!("Número de vogais: {}", contador);
}
