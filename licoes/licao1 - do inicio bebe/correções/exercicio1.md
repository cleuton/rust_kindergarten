O código correto é esse: 

```rust
fn main() {
    // Isso dá erro: println!((5+7+3+1+2+10+6)/7)
    // Isso dá resultado errado: 
    // println!("{}",(5+7+3+1+2+10+6) / 7)
    println!("{}",(5+7+3+1+2+10+6) as f64 / 7.0)
}
```