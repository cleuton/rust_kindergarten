<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Vou esconder isso do bebê : Encapsulamento

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

**Aiin Rust não tem encapsulamento***

Não, isso tá errado, Bebê! Rust tem encapsulamento sim, mas o mecanismo é diferente do que muita gente está acostumada em linguagens orientadas a objetos clássicas como Java ou C++.

Em Rust o controle de visibilidade é feito com o sistema de módulos (`mod`) e o modificador `pub`. Por padrão, tudo é privado ao módulo em que foi declarado. Só fica acessível de fora se você marcar como `pub`. Isso vale para structs, enums, funções, traits, constantes e também para os campos das structs.

Por exemplo, se você declara:

```rust
mod pessoa {
    pub struct Pessoa {
        nome: String,        // privado
        pub idade: u32,      // público
    }

    impl Pessoa {
        pub fn new(nome: String, idade: u32) -> Pessoa {
            Pessoa { nome, idade }
        }
    }
}

fn main() {
    let p = pessoa::Pessoa::new("Ana".into(), 30);
    println!("{}", p.idade);   // ok
    // println!("{}", p.nome); // erro: campo privado
}
```

Aqui o campo `nome` está encapsulado, porque não pode ser acessado diretamente de fora do módulo, mas `idade` pode.

Então Rust tem sim encapsulamento, mas não existe um sistema de `private/protected/public` com herança. O que existe é um controle de visibilidade baseado em módulos, que cobre bem o papel de encapsulamento.

Em Rust, um `mod` é como se fosse uma caixinha onde você guarda código relacionado: structs, funções, traits, enums. Por padrão, tudo que você coloca nessa caixinha fica escondido lá dentro, e só sai se você disser `pub`.

Se fosse Java, um `mod` é parecido com um `package`, mas sem a burocracia de ter que colocar aquele cabeçalho chato no começo do arquivo. É como se em Java você dissesse: *"ok, tudo que está entre essas chaves aqui pertence a esse pacote"*.

O detalhe engraçado é que em Java, se você não declarar nada, as coisas viram “package-private” sem ninguém entender direito o que isso significa. Em Rust, se você não falar nada, é como se tivesse trancado sua caixinha (`mod`) e só você tem a chave. Quer dividir com o mundo? Coloca um `pub` na tampa e pronto.

## Mod vs arquivo

Um `mod` em Rust não é obrigatoriamente um arquivo, é um **módulo**. Ele é apenas uma unidade de organização do código. Você pode declarar um módulo dentro de outro módulo, tudo no mesmo arquivo, usando `mod nome { ... }`. Mas também pode “tirar” esse bloco de chaves e colocar o conteúdo em outro arquivo. Aí o compilador entende que aquele arquivo corresponde ao módulo.

Exemplo:

```rust
// main.rs
mod util; // diz: existe um módulo chamado util, está em util.rs

fn main() {
    util::oi();
}
```

```rust
// util.rs
pub fn oi() {
    println!("oi!");
}
```

Ou seja: **um arquivo pode ser um módulo**, mas nem todo módulo precisa ser um arquivo (pode estar embutido).

Sobre visibilidade: por padrão, tudo dentro de um módulo é **privado ao módulo onde foi definido**. Mas dentro do **mesmo módulo** (ou seja, o mesmo arquivo, se ele for o módulo), todas as funções, structs e campos se enxergam. Não existe essa coisa de “público só para o arquivo”. O limite de visibilidade é o **módulo**, e não o arquivo. O arquivo só é um detalhe de como você organiza os módulos no disco.

Resumindo: `mod` é um módulo lógico; um arquivo pode virar um módulo; dentro de um módulo, tudo é acessível; para fora, só com `pub`.


## Exercícios

Agora vamos criar um programa que calcule raízes de equação do segundo grau, mas vamos usar módulos. 

1) Crie um módulo `equacao` com um `enum` **Raizes** e uma função `calcular()` públicos. 
2) Crie uma função privada `delta()` nesse módulo. 
3) O `enum` **Raizes** reve ter as variantes: `Uma` com uma raíz `f64`, `Duas` com duas raízes `f64` e `Nenhuma` sem raízes.
4) A função `calcular()` recebe os três coeficientes (a,b,c) como `f64` e retorna uma instância do `enum` `Raizes`.
5) A função `calcular()` invoca uma função privada `delta()` dentro do módulo, que recebe os coeficientes e retorna `Result<f64, String>`, ou seja, se o delta for negativo, retorna uma mensagem de erro, caso contrário, retorna `Ok<Delta>`.

Use `match` na `main()`. Crie um loop para calcular várias equações. Por exemplo: 

```text
x2−5x+6=0 x1=2, x2=3
x2−4x+4=0 x=2
x2+x+1=0 nenhuma raiz
```

Para acessar funções e elementos dentro de um módulo temos que usar a sintaxe do caminho completo (**turbofish sintax**): 

`equacao::Raizes::Duas{x1,x2}`

Ou seja: `modulo::Elemento::Sub-elemento`.

Se quiser simplificar, pode utilizar o comando `use`: 

```rust
use equacao::{Raizes, calcular};
```

E não precisa escrever todo o `turbofish`. Saca só: 

```rust
use equacao::{Raizes, calcular};
mod equacao {
    pub enum Raizes {
        Uma {x: f64},
        Duas {x1: f64, x2: f64},
        Nenhuma,
    }
    ...
}
fn main() {
    let equacoes = vec!((1.0,-5.0,6.0), (1.0,-4.0,4.0), (1.0,1.0,1.0));
    for eq in equacoes {
        match calcular(eq.0,eq.1,eq.2) {
            Raizes::Duas{x1,x2} => println!("a: {}, b:{}, c:{} : x1={}, x2={}",eq.0,eq.1,eq.2,x1,x2),
    ...
```

Nesse exemplo eu criei um `Vec` de tuplas. Em Rust, uma tupla é um valor que junta diferentes dados em uma única estrutura, usando parênteses e separando por vírgula. Ela pode misturar tipos diferentes e o acesso é feito pela posição, começando em zero. Por exemplo: `let pessoa = ("Ana", 30, 1.65);` cria uma tupla com um `&str`, um `i32` e um `f64`. Para acessar, você usa `pessoa.0`, `pessoa.1`, `pessoa.2`. É como um pacote rápido de valores, útil quando você quer devolver mais de um resultado de uma função sem precisar criar uma struct.
