```rust
#[derive(Debug)]
struct Ponto<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Ponto {x: 5, y: 10};
    let p2 = Ponto {x: 10.5, y: 4.5};
    println!("Ponto p1: {:?}", p1);
    println!("Ponto p2: {:?}", p2);
}
```