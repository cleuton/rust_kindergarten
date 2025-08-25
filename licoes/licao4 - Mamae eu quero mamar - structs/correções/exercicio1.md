Aqui está o exercício1 já corrigido: 

```rust
#[derive(Debug)]
struct Pessoa {
    id: i32,
    nome: String,
    email: String,
}

fn main() {
   let p = Pessoa {
   		id: 5,
   		nome: "Fulano".to_string(),
   		email: "fulano@teste.com".to_string()
   		};
   println!("Pessoa: {:?}", p);
}
```