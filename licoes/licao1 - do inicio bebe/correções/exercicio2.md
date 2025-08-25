Correção: 

```rust
fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media = (nota1+nota2+nota3)/3.0;
   println!("A média é: {}",media);
}
```

As variáveis são todas imutáveis. Você pode atribuir um valor inicial (caso não tenha sido atribuído), mas não pode mudar: 

```rust
fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media: f64; 
    media = (nota1+nota2+nota3)/3.0;
   println!("A média é: {}",media);
}
```

Se adicionarmos 1 à média: 

```rust
fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media = (nota1+nota2+nota3)/3.0;
   media = media + 1;
   println!("A média é: {}",media);
}
```

Vamos tomar um erro: error[E0384]: cannot assign twice to immutable variable `media` pois ela é imutável!