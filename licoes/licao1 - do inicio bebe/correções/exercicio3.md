O correto é especificar os tipos de dados dos argumentos e o do valor de retorno (se a função retornar algo): 

```rust
fn calcular(n1: f64, n2: f64, n3: f64) -> f64 {
   return (n1 + n2 + n3) / 3.0;
}

fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media = calcular(nota1, nota2, nota3);
   println!("A média é: {}",media);
}
```
