<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Tendi não Tia : Generics

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Generics é uma praga que existe em muitas linguagens como: **Java, Go, Rust, C++ (template class) etc**. Para que serve essa porcaria? Permite que funções, classes ou tipos trabalhem com múltiplos tipos de dados de forma segura e sem duplicação de código. Questionável, mas existe. 

Em Rust, generics são como moldes de bolo: você escreve a forma uma vez e o compilador assa bolos diferentes para cada ingrediente que você pedir. Se você tem uma função `somar<T>(a: T, b: T)`, o compilador gera um código de verdade para `i32`, outro para `f64`, outro para o que você usar. Isso se chama **monomorfização**: no fim, o código fica especializado e rápido, sem perder segurança.

No Java, generics parecem moldes também, mas é só ilusão de ótica. O compilador faz *type erasure*: ele usa os tipos na hora da checagem, mas depois troca tudo por `Object` (ou equivalente). É como se você tivesse um molde universal que serve para qualquer massa, mas na hora de assar todo bolo passa pelo mesmo forno genérico. Funciona, mas você não tem bolos otimizados para cada sabor, só um truque com “casting”.

Resumindo com humor:

* **Rust**: “Vou te dar bolos diferentes, cada um feito do jeito certo.”
* **Java**: “Vou fingir que são bolos diferentes, mas na verdade é tudo massa de `Object` disfarçada.”

```rust
// Uma função genérica que só devolve o que recebeu
fn identidade<T>(valor: T) -> T {
    valor
}

fn main() {
    let numero = identidade(42);       // aqui T é i32
    let texto = identidade("olá");     // aqui T é &str

    println!("numero: {}, texto: {}", numero, texto);
}
```

Perfeito, vamos deixar lifetimes de fora por enquanto.
Então, falando só de **usos avançados de generics com traits** em Rust, sem complicar:

---

### 1. Bounds em Traits

Você pode restringir o tipo genérico para que só funcione se ele implementar certo trait.

```rust
use std::fmt::Display;

fn repetir<T: Display>(valor: T) {
    println!("{0} - {0}", valor);
}
```

Aqui `T` só funciona para tipos que “sabem se mostrar” (implementam `Display`).

---

### 2. `impl Trait`

Um atalho para não ter que dar nome ao tipo genérico:

```rust
fn cria_lista() -> impl Iterator<Item = i32> {
    1..5
}
```

A função devolve “algum tipo que implementa `Iterator<Item=i32>`”, sem precisar dizer qual.

---

### 3. Implementando traits em tipos genéricos

Você pode dizer que qualquer `Vec<T>` tem um certo comportamento:

```rust
trait Contagem {
    fn conta(&self) -> usize;
}

impl<T> Contagem for Vec<T> {
    fn conta(&self) -> usize {
        self.len()
    }
}
```

---

### 4. Associated Types

Um trait pode “carregar” tipos junto dele, sem você precisar passar como parâmetro toda hora:

```rust
trait Conversor {
    type Saida;
    fn converte(&self) -> Self::Saida;
}

struct Inteiro(i32);

impl Conversor for Inteiro {
    type Saida = String;
    fn converte(&self) -> String {
        self.0.to_string()
    }
}
```

Aqui, `Conversor` define que cada implementação escolhe o tipo de saída.

---

### 5. Blanket impls

Você pode dar poderes extras para qualquer tipo que implemente outro trait:

```rust
use std::fmt::Display;

trait EmCaixa {
    fn caixa(&self) -> String;
}

impl<T: Display> EmCaixa for T {
    fn caixa(&self) -> String {
        format!("[{}]", self)
    }
}

fn main() {
    println!("{}", 42.caixa());
    println!("{}", "olá".caixa());
}
```

Qualquer coisa que implemente `Display` agora também tem `.caixa()`.


## Exercícios

1) Implemente uma função genérica minimo que recebe dois valores e retorna o menor deles ou None, caso sejam iguais (use Option). Para poder usar `<>`, o tipo `T` precisa implementar o trait `PartialOrd`.

```rust
fn menor<T: PartialOrd>(v1: T, v2: T) -> Option<T> {}
```

2) Crie uma struct genérica Ponto com dois campos (x e y) que podem ter qualquer tipo. Depois, crie uma instância com inteiros e outra com números de ponto flutuante. Não se esqueça que a struct tem que implementar `Debug` e no `println!` tem que usar `{:?}`!

3) Implemente uma função genérica troque que recebe uma tupla (a, b) e devolve (b, a).