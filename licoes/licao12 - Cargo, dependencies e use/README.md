<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Chamêgo de Vó - Cargo, dependências e use

[**VÍDEO**](https://youtu.be/ZNejup89Gmg).

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

**Rust** tem um mecanismo de controle de dependências sim. Você pode referenciar, baixar e compilar quaisquer dependências que o seu programa precisa, além de gerenciar o seu **projeto Rust**. 

> **Cargo:** ferramenta de gerenciamento de projetos da linguagem Rust: ele cuida da compilação do código, da execução de testes, da geração de documentação, do download de dependências (outras bibliotecas) e do gerenciamento de versões, tudo com comandos simples como `cargo build`, `cargo run` ou `cargo test`. É como um "make", "npm", "maven" ou "pip" do Rust, mas integrado oficialmente ao ecossistema, facilitando muito o desenvolvimento, organização e compartilhamento de projetos. Você já o tem instalado na sua máquina, pois ao instalar o `rustc` instalou todo o `toolchain`.

Dependências em Rust são códigos de fora do seu projeto que você usa, como bibliotecas para ler arquivos CSV, fazer validações ou usar expressões regulares.

Sem o Cargo, você tem que compilar tudo na mão com o comando `rustc`. Se o seu código depende de uma biblioteca, como `regex`, você precisa baixar o código dela e de todas as bibliotecas que ela depende, compilar cada uma com `rustc --emit obj` ou `--emit lib`, gerar arquivos `.rlib`, e depois compilar o seu código passando todas essas bibliotecas com `--extern`. Por exemplo:

```bash
rustc --crate-type lib --emit rlib regex_source.rs  # Compila a biblioteca
rustc --extern regex=libregex.rlib meu_codigo.rs  # Usa no seu código
```

Isso é extremamente trabalhoso, porque você tem que resolver manualmente todas as dependências, versões e ordem de compilação. Na prática, ninguém faz isso.

Com o Cargo, tudo é automático. Você cria um projeto com `cargo new meu_projeto`, abre o arquivo `Cargo.toml` e adiciona as dependências:

```toml
[dependencies]
regex = "1.10"
csv = "1.3"
```

Depois, no seu código `src/main.rs`, você usa:

```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    println!("{}", re.is_match("123"));
}
```

E roda `cargo build`. O Cargo baixa `regex`, `csv` e todas as dependências delas, compila na ordem certa e gera o executável. Ele também evita duplicação e resolve versões compatíveis. É a forma correta e simples de trabalhar com dependências em Rust.

## Criando um novo projeto

Para criar um novo projeto, vá para a pasta onde deseja criar seu projeto e digite: 

```bash
cargo new <nome-do-projeto>
```

Ele vai: 

1. Criar uma pasta com o nome do projeto.
2. Criar um arquivo `Cargo.toml` de configuração do projeto.
3. Criar uma pasta `src`. 
4. Criar um arquivo `main.rs` dentro de `src`.

Depois de modificar o código, é só rodar: 

```bash
cargo run
```

Ele baixará, compilará as dependências e o seu código-fonte, executando em seguida. Nesse momento, será criada uma pasta `target`, com o executável, e um arquivo `Cargo.lock` com o **travamento** das versões das dependências utilizadas. 

> O arquivo `Cargo.lock` é gerado automaticamente pelo Cargo e contém uma lista exata de todas as versões das dependências usadas no projeto, incluindo crates diretas e indiretas, com seus hashes e fontes. Ele garante que, em qualquer ambiente (seja na sua máquina, na de um colega ou em produção) as mesmas versões exatas sejam usadas, o que torna a compilação reprodutível e evita que atualizações automáticas de dependências introduzam mudanças inesperadas ou quebras. Enquanto o `Cargo.toml` define as versões aceitáveis (por exemplo, "1.0 ou compatível"), o `Cargo.lock` "congela" essas escolhas, funcionando de forma semelhante a um `package-lock.json` no npm ou `Pipfile.lock` no Python. Ele deve ser commitado em projetos executáveis (binários), mas geralmente não em bibliotecas.

## Cargo.toml

O `Cargo.toml` é o **arquivo de configuração de um projeto Rust**. Ele diz ao Cargo (o gerenciador de pacotes e build do Rust) como compilar e quais dependências usar.

As principais configurações são:

* `[package]` → informações do projeto: `name`, `version`, `edition`, `authors`, `license`.
* `[dependencies]` → bibliotecas externas que seu projeto usa (ex: `serde = "1.0"`).
* `[dev-dependencies]` → dependências usadas só em testes e benchmarks.
* `[features]` → ativa ou desativa partes opcionais do código.
* `[workspace]` → quando você tem vários crates dentro de um mesmo repositório.

Resumindo: o `Cargo.toml` é como o “manifesto” do seu projeto Rust, centralizando nome, versão, dependências e opções de build.

Aqui vai um exemplo bem curto de um `Cargo.toml` com as partes principais:

```toml
[package]
name = "meu_projeto"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome <seu@email.com>"]
license = "MIT"

[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[features]
default = ["sqlite"]
sqlite = ["rusqlite"]

[workspace]
members = [
    "crates/core",
    "crates/api",
]
```

Explicando rápido:

* `[package]` → define metadados do crate.
* `[dependencies]` → bibliotecas usadas no código normal.
* `[dev-dependencies]` → só para testes/benchmarks.
* `[features]` → recursos opcionais que você pode ativar com `--features`.
* `[workspace]` → organiza múltiplos crates dentro do mesmo repositório.

## Como procurar dependências

Você pode usar o [`crates.io`](https://crates.io) que é o repositório de dependências do **Rust**. Você pode criar bibliotecas e postar lá também!

## O comando `use`

Em Rust, o `use` não inclui código novo no seu projeto, apenas cria um atalho para itens já disponíveis no escopo, como módulos, funções ou tipos. O código já está presente porque foi compilado como parte de sua dependência ou do seu próprio projeto, e `use` apenas facilita o acesso a ele.

```rust
use rand::Rng;
```

Em Java, o `import` funciona de forma semelhante: ele não carrega classes ou código, apenas permite que você se refira a classes de outros pacotes pelo nome curto em vez do nome completo. Por exemplo, `import java.util.ArrayList;` permite usar `ArrayList` diretamente em vez de `java.util.ArrayList`. A classe já está disponível na JVM ou no classpath; o `import` só afeta o nome usado no código.

Em C e C++, o `#include` é diferente: ele realmente insere o conteúdo do arquivo de cabeçalho (como `stdio.h` ou `vector`) diretamente no código-fonte antes da compilação. Isso pode adicionar muitas linhas ao seu código durante a pré-processamento, mesmo que você não use todos os elementos incluídos. É uma inclusão textual, não apenas um atalho de escopo.

Em Python, `import` carrega o módulo especificado em tempo de execução (ou no momento da execução do script), executando seu código e tornando-o acessível. Isso significa que o módulo é efetivamente "incluído" no sentido de que ele é carregado na memória e pode ter efeitos colaterais (como execução de código global). O `from ... import ...` funciona como o `use` do Rust, trazendo nomes para o escopo atual.

Em JavaScript (ES6+), `import` é estático e declarativo: ele não executa ou copia código, mas estabelece uma ligação com funções, objetos ou primitivos exportados por outro módulo. O módulo importado é avaliado uma vez, e as importações são referências a esses valores. Assim como em Rust, o `import` só organiza o acesso, sem duplicar ou inserir código diretamente.

## Caminho de módulos

Você tem razão, ficou mais complexo do que precisava. Vou simplificar de verdade.

Em Rust, um **caminho de módulo** é só o "endereço" de algo dentro do código, usando `::` como separador.

Exemplo:

```rust
let mut mapa = std::collections::HashMap::new();
```

Aqui o caminho é `std::collections::HashMap`. Ele vai entrando em cada módulo (`std`, depois `collections`) até chegar no item (`HashMap`).

Você também pode usar `crate::` para começar do seu próprio projeto:

```rust
let repo = crate::repository::InMemoryRepo::new();
```

Ou encurtar com `use`:

```rust
use std::collections::HashMap;
let mut mapa: HashMap<String, i32> = HashMap::new();
```

Resumindo, o `use` serve para renomear ou simplificar caminhos de módulos que você colocou nas dependências do seu projeto.

## Exercícios

Crie um programa **Rust** que simule um dado (6 números possíveis), selecionando de maneira aleatória a cada chamada. Para isto, você usará a biblioteca `rand`, na versão `0.9.2`. Veja no `crates.io` como usar essa biblioteca para obter um `range`. 

Você terá que instanciar um `trait Rng`. Felizmente, no módulo `rand` temos uma função `rng()` que cria e retorna uma instância desse `trait`, e ela terá o método `random_range(inferior..superior)` que gera um número aleatório no intervalo fechado em inferior e aberto em superior. 

Use o `Cargo` para gerar o projeto, complemente o `Cargo.toml` com a dependência, e altere o `src/main.rs` para gerar e mostrar o número.

