Código original que você digitou: 

```rust
#[derive(Debug)]
struct Aluno {
    matricula: i32,
    nome: String,
}

impl Aluno {
    fn novo(matricula: i32, nome: String) -> Self {
        Aluno{matricula, nome}
    }
    // Era melhor implementar Display, mas vamos simplificar
    fn mostrar(self) {
        println!("{:?}", self);
    }
}

fn main() {
    let a = Aluno::novo(55, "Fulano".to_string());
    a.mostrar();
}
```

Código corrigido para passar referência: 

```rust
#[derive(Debug)]
struct Aluno {
    matricula: i32,
    nome: String,
}

impl Aluno {
    fn novo(matricula: i32, nome: String) -> Self {
        Aluno{matricula, nome}
    }
    // Era melhor implementar Display
    fn mostrar(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let a = Aluno::novo(55, "Fulano".to_string());
    a.mostrar();
    println!("Matrícula: {}", a.matricula);
}
```