A correção é essa: 

```rust
fn calcular(numeros: [f64;5]) -> f64 {
    let mut soma = 0.0;
    for n in numeros {
    	soma += n;
    }
    return soma / numeros.len() as f64;
}

fn main() {
   let v = [4.0, 7.0, 10.0, 1.0, 3.0];
   // Poderia ser assim: let v: [f64;5] = [4.0, 7.0, 10.0, 1.0, 3.0];
   let media = calcular(v);
   println!("A média é: {}", media);
}
```