Versão que vai dar erro de compilação: 

```rust
#[derive(Debug)]
struct Pessoa {
    id: i32,
    nome: String,
    email: String,
}

fn mostrar(ps: Pessoa) {
    println!("Mostrando: {:?}", ps);
}

fn main() {
   let mut p = Pessoa {
   		id: 5,
   		nome: "Fulano".to_string(),
   		email: "fulano@teste.com".to_string()
   		};
   println!("Pessoa: {:?}", p);
   mostrar(p);
   p.nome = "Beltrano".to_string();
   println!("Pessoa: {:?}", p);   
}
```

Agora, vamos transformar o argumento `ps` da função em uma referência: 

```rust
#[derive(Debug)]
struct Pessoa {
    id: i32,
    nome: String,
    email: String,
}

fn mostrar(ps: &Pessoa) {
    println!("Mostrando: {:?}", ps);
}

fn main() {
   let mut p = Pessoa {
   		id: 5,
   		nome: "Fulano".to_string(),
   		email: "fulano@teste.com".to_string()
   		};
   println!("Pessoa: {:?}", p);
   mostrar(&p);
   p.nome = "Beltrano".to_string();
   println!("Pessoa: {:?}", p);   
}
```
