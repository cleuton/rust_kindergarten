```rust
use rand::Rng;

fn main() {
	let mut rng = rand::rng();
	let valor: u8 = rng.random_range(1..7);
	println!("Valor do dado: {}", valor);
}
```

```toml
[package]
name = "dado"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.9.2"

```