Eis a forma mais simples: 

```rust
fn main() {
	let v = vec![4.0, 7.0, 10.0, 1.0, 3.0];
	let mut soma = 0.0;
	for valor in &v {
		soma += valor;
	}
	println!("Média: {}", soma / v.len() as f64);
}
```

E se for necessário pegar o índice? 

```rust
fn main() {
	let v = vec![4.0, 7.0, 10.0, 1.0, 3.0];
	let mut soma = 0.0;
    for (i, &valor) in v.iter().enumerate() {
        println!("Índice: {}, Valor: {}", i, valor);
        soma += valor;
    }    
	println!("Média: {}", soma / v.len() as f64);
}
```