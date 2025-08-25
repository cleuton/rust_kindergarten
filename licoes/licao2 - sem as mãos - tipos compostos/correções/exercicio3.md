Essa versão não funciona: 

```rust
fn tamanho(s: String) -> usize {
    return s.chars().count();
}

fn main() {
    let nome = "Fulano de Tal".to_string();
    let tam_nome = tamanho(nome);
    println!("{} {}", nome, tam_nome);
}
```

Ao invocar a função `tamanho()` a variável `nome` é movida para o argumento `s` dentro da função. A variável `nome` é inválida e vai dar erro quando tentarmos usar o `println!` com ela. 

Essa versão funciona: 

```rust
fn tamanho(s: &String) -> usize {
    return s.chars().count();
}

fn main() {
    let nome = "Fulano de Tal".to_string();
    let tam_nome = tamanho(&nome);
    println!("{} {}", nome, tam_nome);
}
```

Estamos recebendo uma referência imutável na função `tamanho()` e passamos uma referência imutável para ela na função `main()`.