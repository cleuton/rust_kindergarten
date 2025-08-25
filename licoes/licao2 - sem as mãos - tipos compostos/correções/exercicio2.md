É algo assim: 

```rust
fn main() {
    let mut s1 = "Texto1".to_string(); // Converte "Texto1" para String
    let s2 = &s1; // Emprestimo imutável de s1
    s1.push_str("!"); // Tenta alterar o conteúdo usando s1
    println!("{}", s2); // Mostra o conteúdo usando s2
} 
```

Prestenção! "Texto1" é um `&str`! Uma referência para um `String`! Se você declarar `s1` sem especificar tipo ele vai assumir o tipo do valor atribuído. Ao usarmos `.to_string()` forçamos a conversão para um valor `String`. 