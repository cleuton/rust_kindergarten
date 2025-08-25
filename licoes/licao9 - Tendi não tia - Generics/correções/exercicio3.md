```rust
fn troca<T> (tupla: (T,T)) -> (T,T) {
    (tupla.1, tupla.0)
}
fn main() {
    let tupla = (5,6);
    let outra = troca(tupla);
    println!("Troca: {} e {}",outra.0, outra.1);
}
```