```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let arquivo = "numeros.txt"; 
    let dados = fs::read_to_string(arquivo)?;
    let mut soma = 0.0;
    let mut conta = 0;
    let valores: Vec<&str> = dados.split(",").collect();
    for v in &valores {
        conta += 1;
        if let Ok(num) = v.trim().parse::<f64>() {
            soma += num;
        }
    }
    let media = soma / conta as f64;
    let mut variancia = 0.0;
    for v in &valores {
        if let Ok(num) = v.trim().parse::<f64>() {
            variancia = variancia + (media - num) * (media - num);
        }
    }
    let desvio = (variancia / (conta - 1) as f64).sqrt();
    println!("A média é: {} e o desvio-padrão é: {}", media, desvio);

    Ok(())
}

```