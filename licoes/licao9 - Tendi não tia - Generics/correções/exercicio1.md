```rust
fn menor<T: PartialOrd>(v1: T, v2: T) -> Option<T> {
    if v1 < v2 {
        return Some(v1);
    } else if v2 < v1 {
        return Some(v2);
    }
    None
}

fn main() {
    if let Some(m) = menor(10,20) {
        println!("O menor é {}",m);
    } else {
        println!("São iguais");
    }
    if let Some(m) = menor(10,10) {
        println!("O menor é {}",m);
    } else {
        println!("São iguais");
    }

}

```