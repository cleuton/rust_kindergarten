Eis o código correto: 

```rust
fn calcular(numeros: &Vec<f64>) -> f64 {
    let mut soma = 0.0;
    for n in numeros {
    	soma += n;
    }
    return soma / numeros.len() as f64;
}

fn main() {
   let v = vec![4.0, 7.0, 10.0, 1.0, 3.0];
   let media = calcular(&v);
   println!("A média é: {}", media);
}
```

Neste caso em especial não precisaríamos passar uma referência ao Vec para a função calcular, deixando que ela mova o vec para `numeros`, pois não usamos mais o `v` na função `main()` depois de invocar `calcular()`. Então, isso funcionaria: 

```rust
fn calcular(numeros: Vec<f64>) -> f64 {
    let mut soma = 0.0;
    for n in &numeros {
    	soma += n;
    }
    return soma / numeros.len() as f64;
}

fn main() {
   let v = vec![4.0, 7.0, 10.0, 1.0, 3.0];
   let media = calcular(v);
   println!("A média é: {}", media);
}  	
```

Mas precisaremos usar uma referência à `numeros` no `for`. Esta é uma prática ruim, pois se precisarmos usar `v` na função `main()` vamos tomar um erro pois ela teria sido MOVIDA.
