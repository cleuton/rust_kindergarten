use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let resto = args[1].parse::<u32>().unwrap() % 2;
    if resto == 0 {
        println!("O número é par");
    } else {
        println!("O número é ímpar");
    }
}
