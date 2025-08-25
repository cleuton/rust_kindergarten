A correção é essa: 

```rust
fn tamanho(s: &String) -> usize {
    return s.chars().count();
}

fn main() {
    let nome = String::from("Fulano de Tal");
    let tam_nome = tamanho(&nome);
    println!("{} {}", nome, tam_nome);
    let s2 = &nome;
    println!("{} {}", nome, s2);
}
```

Não deve dar erro pois houve empréstimos e não movimentações. 