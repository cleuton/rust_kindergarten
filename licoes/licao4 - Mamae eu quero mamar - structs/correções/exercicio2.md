Essa é a correção do exercício2: 

```rust
#[derive(Debug)]
struct Pessoa {
    id: i32,
    nome: String,
    email: String,
}

fn main() {
   let mut p = Pessoa {
   		id: 5,
   		nome: "Fulano".to_string(),
   		email: "fulano@teste.com".to_string()
   		};
   println!("Pessoa: {:?}", p);
   p.nome = "Beltrano".to_string();
   println!("Pessoa: {:?}", p);   
}
```
    