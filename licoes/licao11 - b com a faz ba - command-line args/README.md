<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# "b" com "a" faz "ba" : Command-line arguments e arquivos

[**VÍDEO**](https://youtu.be/TkaonTNF4_A).

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Command-line arguments em Rust são muito fáceis de ler: 

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

O resultado desse código seria: 

```bash
./exercicio1 segundo terceiro
[exercicio1.rs:5:5] args = [
    "./exercicio1",
    "segundo",
    "terceiro",
]
```

O primeiro argumento é sempre o nome do programa, do segundo em diante são os argumentos que você passa. Lembrando que vetores em Rust começam em zero.

Não vamos falar sobre databases nem csv e nem json. Vamos falar de arquivos simples, de texto. Aqui está um código que cria um arquivo simples de texto: 

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let caminho = "saida.txt";
    let conteudo = "Oi!\nEste é um arquivo de texto";

    fs::write(caminho, conteudo)?; 

    println!("Gravado em {}", caminho);
    Ok(())
}
```

E este código lê o arquivo, mostrando no terminal: 

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let arquivo = "saida.txt"; 
    let dados = fs::read_to_string(arquivo)?;
    println!("Conteúdo:\n{}", dados);
    Ok(())
}
```


## Exercícios

Dado esse arquivo de texto, calcule a média e desvio padrão amostral dos números: 

```text
10.5, 10.2, 10.8, 10.6, 10.7, 10.2
```

Você vai precisar separar os valores por vírgula e converter em `f64`. Para separar um `String` por um delimitador, você pode utilizar o método `split()` que retorna um `iterator`. Você precisa usar `collect()` para retornar um `Vec`: 

```rust
let fruits_vec: Vec<&str> = text.split(',').collect();
```

E para transformar cada elemento em `f64` você pode utilizar o `parse()` que retorna um `Result<T,E>`: 

```rust
    let s = "3.14159";
    let parsed_f64: f64 = match s.trim().s.parse()::<f64> {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Falha ao analisar o string: {}", e);
            0.0 
        }
    };
```

Para calcular o desvio-padrão, calcule primeiro a variância, que é o somatório dos quadrados das diferenças entre a média e cada número, depois divida pela quantidade de números menos 1 (desvio-padrão amostral).