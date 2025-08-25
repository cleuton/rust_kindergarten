O código para criar e navegar no vetor é este: 

```rust
fn main() {
	let v = [4.0, 7.0, 10.0, 1.0, 3.0];
	let mut soma = 0.0;
	/*
	for valor in v {
	    soma += valor;
	}
	*/
	for x in 0..v.len() {
		soma += v[x];
	}
	println!("Média: {}", soma / v.len() as f64);
}
```

Há duas maneiras de navegar: Uma sem índice e outra utilizando índice. 