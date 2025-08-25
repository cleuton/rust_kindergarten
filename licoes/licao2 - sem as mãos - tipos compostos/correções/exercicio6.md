Esse código dá erro: 

```rust
fn main() {
    let mut s1 = String::from("Texto");
    let s2 = & mut s1;
    println!("{}",s1);
    let s3 = & s1;
    println!("{}",s3);
    println!("{}",s2);
}
```

Podemos retirar a mutabilidade de `s2`: 

```rust
fn main() {
    let mut s1 = String::from("Texto");
    let s2 = & s1; // Aqui
    println!("{}",s1);
    let s3 = & s1;
    println!("{}",s3);
    println!("{}",s2);
}
```

Ou podemos mover o `println!` de `s1` e a atribuição de `s3` para depois do `println!` de `s2`: 

```rust
fn main() {
    let mut s1 = String::from("Texto");
    let s2 = & mut s1;

    println!("{}",s2); // Depois dessa linha s2 sai do escopo por otimização do compilador.

    println!("{}",s1);
    let s3 = & s1;
    println!("{}",s3);
}
```