<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Talquinho no B.: Enums

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

**enum** em **Rust** é muito mais relevante do que em outras linguagens de programação.

O `enum` em Rust é muito mais poderoso e relevante do que em outras linguagens. Enquanto em C ou Java ele serve apenas para definir constantes nomeadas, em Rust é um tipo algébrico que pode armazenar dados diferentes em cada variante, tornando-o essencial para modelar dados, erros e estados.

### 1. `enum` com dados associados

Cada variante pode conter dados distintos:

```rust
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

fn processar(m: Mensagem) {
    match m {
        Mensagem::Sair => println!("Saindo"),
        Mensagem::Mover { x, y } => println!("Mover para ({}, {})", x, y),
        Mensagem::Escrever(texto) => println!("Texto: {}", texto),
        Mensagem::MudarCor(r, g, b) => println!("Cor: ({}, {}, {})", r, g, b),
    }
}
```

`match` é como o `switch` do Java, só que funciona sobre `enums`. Todas as variantes do `enum` devem ser tratadas, ou então deve-se acrescentar um `_` (underscore).

### 2. `Option<T>`: substitui valores nulos

Evita erros comuns como `null`:

```rust
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match dividir(8.0, 2.0) {
        Some(resultado) => println!("Resultado: {}", resultado),
        None => println!("Divisão por zero"),
    }
}
```

Podemos simplificar e em vez de usar `match` usar `if-let`: 

```rust
// usando if let para simplificar
if let Some(resultado) = dividir(8.0, 2.0) {
    println!("Resultado: {}", resultado);
} else {
    println!("Divisão por zero");
}
```

### 3. `Result<T, E>`: tratamento de erros

Usado para operações que podem falhar:

```rust
use std::fs::File;
use std::io::{self, Read};

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}

fn main() {
    match ler_arquivo("dados.txt") {
        Ok(texto) => println!("{}", texto),
        Err(erro) => println!("Erro: {}", erro),
    }
}
```

Repare que a função `ler_arquivo` tem alguns `?`. Vamos explicar o que são. A função retorna um `Result<String, io::Error>`, ou seja, pode devolver o conteúdo do arquivo dentro de `Ok`, ou um erro dentro de `Err`. Isso existe porque operações de arquivo podem falhar. O operador `?` serve para simplificar esse tratamento: se a operação deu certo, ele pega o valor e segue o código; se deu errado, a função para e retorna o erro automaticamente. No seu exemplo, cada `?` evita que você tenha que escrever `match` a cada passo, deixando o código mais limpo.


Podemos utilizar `if-let`: 

```rust
// Simplificando com if let
if let Ok(texto) = ler_arquivo("dados.txt") {
    println!("{}", texto);
} else {
    println!("Erro ao ler o arquivo");
}
```

E se quisermos otimizar, podemos fazer a função `main()` propagar o erro: 

```rust
fn main() -> Result<(), std::io::Error> {
    let texto = ler_arquivo("dados.txt")?;
    println!("{}", texto);
    Ok(())
}
```

Nem precisamos de `if-let` pois o `?` já **desembrulha** o texto, caso o retorno de `ler_arquivo()` tenha sido `OK`, caso contrário, o erro será retornado pela função `main()`.


### 4. Modelagem de estados

Representa mudanças de estado de forma segura:

```rust
enum Estado {
    Desconectado,
    Conectando,
    Conectado { ip: String, porta: u16 },
    Erro(String),
}

fn exibir(estado: Estado) {
    match estado {
        Estado::Desconectado => println!("Desconectado"),
        Estado::Conectando => println!("Conectando..."),
        Estado::Conectado { ip, porta } => {
            println!("Conectado em {}:{}", ip, porta)
        }
        Estado::Erro(m) => println!("Erro: {}", m),
    }
}
```

### 5. Métodos em `enum`

Pode ter implementações com `impl`:

```rust
enum Moeda {
    Centavo,
    CincoCentavos,
    DezCentavos,
    VinteCincoCentavos,
}

impl Moeda {
    fn valor(&self) -> u8 {
        match self {
            Moeda::Centavo => 1,
            Moeda::CincoCentavos => 5,
            Moeda::DezCentavos => 10,
            Moeda::VinteCincoCentavos => 25,
        }
    }
}

fn main() {
    let moeda = Moeda::DezCentavos;
    println!("Valor: {} centavos", moeda.valor());
}
```

Em Rust, `enum` vai muito além de uma lista de constantes. É uma ferramenta central para escrever código seguro e expressivo, usada em tipos como `Option` e `Result`, que evitam erros comuns. Sua capacidade de armazenar dados, modelar estados e exigir tratamento completo nas expressões `match` torna o `enum` fundamental na linguagem.

## Exercícios

Agora vamos criar um programa que calcule troco em centavos utilizando esse algoritmo: 

```texto
algoritmo "Calcular Troco em Moedas"

var
    troco: inteiro
    moedas: vetor[1..6] de inteiro
    valores: vetor[1..6] de inteiro
    i, quantidade: inteiro

inicio
    // Ler o valor do troco em centavos
    escreva("Digite o valor do troco em centavos: ")
    leia(troco)

    // Definir os valores das moedas em ordem decrescente
    valores[1] <- 100  // 1 real
    valores[2] <- 50   // 50 centavos
    valores[3] <- 25   // 25 centavos
    valores[4] <- 10   // 10 centavos
    valores[5] <- 5    // 5 centavos
    valores[6] <- 1    // 1 centavo

    // Inicializar contador de moedas
    para i de 1 ate 6 faca
        moedas[i] <- 0
    fimpara

    // Calcular o número de cada moeda
    para i de 1 ate 6 faca
        se troco >= valores[i] entao
            quantidade <- troco / valores[i]  // divisão inteira
            moedas[i] <- quantidade
            troco <- troco - (quantidade * valores[i])
        fimse
    fimpara

    // Exibir resultado
    escreva("Troco em moedas:")
    para i de 1 ate 6 faca
        se moedas[i] > 0 entao
            se valores[i] = 100 entao
                escreva(moedas[i], " moeda(s) de 1 real")
            senao
                escreva(moedas[i], " moeda(s) de ", valores[i], " centavo(s)")
            fimse
        fimse
    fimpara

fimalgoritmo
```

Para começar, precisamos criar um `enum` para a Moeda, com uma implementação, muito parecido com o que vimos no texto da lição. 

Eis o enum: 

```rust
enum Moeda {
    UmCentavo,
    CincoCentavos,
    DezCentavos,
    VinteEcincoCentavos,
    CinquentaCentavos,
    UmReal
}
```

E a implementação: 

```rust
impl Moeda {
    fn valor(&self) -> i32 {
        match self {
            Moeda::UmCentavo => 1,
            Moeda::CincoCentavos => 5,
            Moeda::DezCentavos => 10,
            Moeda::VinteEcincoCentavos => 25,
            Moeda::CinquentaCentavos => 50,
            Moeda::UmReal => 100,
        }
    }
}
```

Esse método `valor()` retorna o valor da moeda, para ser utilizado nos cálculos. Agora, precisamos de um método no Enum que, dado um índice, retorne qual é a moeda. Precisamos disso pois vamos usar um array para acumular os resultados de divisão por moeda: 

```rust
    fn por_indice(indice: usize) -> Self {
        match indice {
            5 => Moeda::UmCentavo,
            4 => Moeda::CincoCentavos,
            3 => Moeda::DezCentavos,
            2 => Moeda::VinteEcincoCentavos,
            1 => Moeda::CinquentaCentavos,
            0 => Moeda::UmReal,
            _ => Moeda::UmCentavo, // O correto seria retornar Option e None
        }
    }
```

Agora, crie uma função para calcular o troco, que vá dividindo o valor em centavos pelas moedas, da maior para a menor, e vá guardando o resultado no array de moedas. 

Na função `main()` você vai descarregar o array de moedas retornado pela função. 