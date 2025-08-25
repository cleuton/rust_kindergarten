fn primo(n: u32) -> Vec<bool> {
    let mut primos: Vec<bool> = vec![];
    let limite = (n as f32).sqrt() as u32;
    for _ in 0..n+1 {
        primos.push(true);
    }
    primos[0] = false;
    primos[1] = false;
    for i in 2..limite+1 {
        if primos[i as usize] {
            for j in (i*i..n+1).step_by(i as usize) {
                primos[j as usize] = false;
            }
        }
    }
    primos
}

fn fatorar(n: u32) -> Vec<u32> {
    let primos = primo(n+1);
    let mut fatores: Vec<u32> = Vec::new();
    let mut numero = n;
    let mut i: usize = 2;
    while numero > 1 {
        if primos[i as usize] {
            if numero % i as u32 == 0 {
                fatores.push(i as u32);
                numero = numero / i as u32;
            } else {
                i += 1;
                if i >= primos.len() {
                    break;
                }
            }  
        } else {
            i += 1;
            if i >= primos.len() {
                break;
            }
        }
    }
    fatores
}

fn main() {
    println!("Fatores de 18: {:?}", fatorar(18));
    println!("Fatores de 37: {:?}", fatorar(37));
    println!("Fatores de 129: {:?}", fatorar(129));
}
