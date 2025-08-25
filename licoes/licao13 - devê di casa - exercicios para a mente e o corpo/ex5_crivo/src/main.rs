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

fn main() {
    let primos = primo(10);
    for numero in 0..primos.len() {
        if primos[numero] {
            println!("{} é primo", numero);
        } else {
            println!("{} não é primo", numero);
        }
    }
}
