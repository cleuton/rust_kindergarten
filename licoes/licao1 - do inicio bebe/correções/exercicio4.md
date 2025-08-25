Aqui está o código: 

```rust
fn calcular(n1: f64, n2: f64, n3: f64, m: & mut f64) {
//                                     ------------ referência mutável
   *m = (n1 + n2 + n3) / 3.0; // Desreferenciando m e atribuindo valor a ela
}

fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let mut media = 0.0; // variável mutável
   calcular(nota1, nota2, nota3, & mut media); // Passando referência mutável para a função
   println!("A média é: {}",media);
}
```