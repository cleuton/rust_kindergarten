```rust
fn procure_nos_dois<'a>(s1: &'a str, s2: &'a str, padrao: &'a str) -> Option<&'a str> {
    let indice = s1.find(padrao);
    match indice  {
        Some(_) => Some(s1),
        None => {
            if let Some(_) = s2.find(padrao) {
                return Some(s2);
            }
            return None;
        }
    }
}

fn main() {
    let texto = "Eis o xpto aqui".to_string();
    let texto2 = "Aqui não tem xp".to_string();
    let texto3 = "nem aqui".to_string();
    println!("Encontrou em {:?}", procure_nos_dois(&texto,&texto2,"xpto"));
    println!("Encontrou em {:?}", procure_nos_dois(&texto2,&texto3,"xpto"));
}
```