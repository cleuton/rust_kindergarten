<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Cadê o meu suquinho? Métodos

[**Vídeo**](https://youtu.be/cDkfrpKciH8?si=QT00U5ICzt-285C8)

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Minha amiga, meu amigo, NÃO EXISTEM MÉTODOS EM RUST! São apenas funções, declaradas em um bloco de implementação da `struct`. Parece muito com **Go**. Veja um exemplo de `struct` com método. Digite este código, compile e execute como `exercicio1`: 

```rust
#[derive(Debug)]
struct Aluno {
    matricula: i32,
    nome: String,
}

impl Aluno {
    fn novo(matricula: i32, nome: String) -> Self {
        Aluno{matricula, nome}
    }
    // Era melhor implementar Display, mas vamos simplificar
    fn mostrar(self) {
        println!("{:?}", self);
    }
}

fn main() {
    let a = Aluno::novo(55, "Fulano".to_string());
    a.mostrar();
}
```

Calma! Para começar, o bloco `impl` deve ter o mesmo nome da `struct` cujos métodos ele está implementando. Depois, temos duas funções: `novo()` e `mostrar()`. A função `novo()` é o que chamamos de **método de classe**, pois ela não recebe um ponteiro para a instância (`self`), e retorna `Self` (com **S** maiúsculo), que representa a instância criada. Seu propósito é criar instâncias, ou seja, algo semelhante a um **construtor** em outras linguagens. 

E a função `mostrar()` é o equivalente a um **método de instância**, já que o primeiro argumento que recebe é `self` (com **s** minúsculo), sendo a instância completa. 

Agora, vamos **pirar o cabeção**? Acrescente essa linha após a instrução `a.mostrar()`: 

```rust
println!("Matrícula: {}", a.matricula);
```

Vai tomar um baita erro: `error[E0382]: borrow of moved value: a` apontando para a linha: `println!("Matrícula: {}", a.matricula);` !

Antes de soltar um **PQP**, pense bem: Você já sabe o motivo! Sim! Vamos analisar: 

```rust
    a.mostrar();
    println!("Matrícula: {}", a.matricula);
```

As mensagens do compilador explicam o que aconteceu: 

```shell
19 |     a.mostrar();
   |       --------- `a` moved due to this method call
20 |     println!("Matrícula: {}", a.matricula);
   |                               ^^^^^^^^^^^ value borrowed here after move
```

Ao invocar a função `mostrar()` (um método associado à instância de `Aluno` apontada pela variável `a`) houve uma MOVIMENTAÇÃO para o argumento da função, que é `(self)`. Agora troque por uma referência: `(&self)`: 

```rust
    fn mostrar(&self) {
        println!("{:?}", self);
    }
```

E vai funcionar! Métodos cujo argumento `self` não é referência, CONSOMEM a instância, portanto, ela é destruída quando o método termina. 

Agora, se você está realmente prestando atenção, deve ter três perguntas martelando a sua cabeça:

1) O que é o `Self` retornado pela função `Aluno::novo()`? Poderíamos simplesmente retornar `Aluno`?
2) O que é o `self` no argumento da função `Aluno::mostrar()`? Por que não tem tipo? É uma variável comum? 
3) Ao mudarmos o argumento `self` para `&self`, na função `Aluno::mostrar()` por que não precisamos chamar a função igualmente com uma referência? (`a.mostrar()`)

Vamos responder todas. Preste atenção: 

Vamos por partes, direto ao ponto:

1. O que é o `Self` retornado pela função `Aluno::novo()`? Poderíamos simplesmente retornar `Aluno`? 

> `Self` (com **S** maiúsculo) é um alias para o tipo que está sendo implementado naquele bloco `impl`. Aqui, `Self` ≡ `Aluno`. Você poderia retornar `Aluno` no lugar de `Self` sem nenhuma diferença prática. Usa-se `Self` por clareza e porque escala melhor quando o tipo é genérico ou muda de nome.

2. O que é o `self` no argumento da função `Aluno::mostrar()`? Por que não tem tipo? É uma variável comum? 

> `self` (com **s** minúsculo) é o “receiver” do método — a instância sobre a qual o método é chamado. Em métodos de Rust ele tem um lugar especial e o tipo vem embutido na forma do parâmetro: pode ser `self` (por valor, move), `&self` (empréstimo imutável) ou `&mut self` (empréstimo mutável). Não é uma variável “comum” com tipo omitido; é uma sintaxe própria de métodos que expande para `self: Self`, `self: &Self` ou `self: &mut Self`.

3. Ao mudarmos o argumento `self` para `&self`, na função `Aluno::mostrar()` por que não precisamos chamar a função igualmente com uma referência? (`a.mostrar()`)

> Porque em chamadas de método, o compilador aplica “auto-referência/auto-deref” para facilitar a vida. Isso não acontece com funções comuns, que não usam o `receiver` `self`. Escrever `a.mostrar()` com assinatura `fn mostrar(&self)` é "açucar sintático", ou seja, a mesma coisa que `Aluno::mostrar(&a)`. O compilador empresta `a` automaticamente conforme o tipo do receiver. Se fosse `fn mostrar(self)`, aí sim `a.mostrar()` moveria `a` e você não poderia usar `a` depois. Com `&self`, você apenas faz um empréstimo imutável e ainda imprime `a.matricula` depois sem problemas.

Calma. Muita calma nessa hora!


## Exercícios

### 2 Crie um programa com duas structs

Você vai criar um programa com duas structs: `Ponto`, com os atributos `x` e `y`, ambos `f64`, e `Retângulo`, com os atributos `superior_esquerdo` do tipo `Ponto` e `inferior_direito` também do tipo `Ponto`. 
 
Vai implementar um método para saber se os retângulos se interceptam. Como saber isso? Em pseudo-código, poderíamos fazer assim: 

```text
// Assumindo que a origem das coordenadas seja o canto inferior esquerdo
tipo Ponto { x, y }
tipo Retangulo { superior_esquerdo: Ponto, inferior_direito: Ponto }

fun retangulos_interceptam(r1: Retangulo, r2: Retangulo) -> bool:
    # Sem interseção horizontal
    se r1.inferior_direito.x <= r2.superior_esquerdo.x: retorne falso
    se r2.inferior_direito.x <= r1.superior_esquerdo.x: retorne falso

    # Sem interseção vertical (origem no canto inferior esquerdo; y cresce para cima)
    se r1.superior_esquerdo.y <= r2.inferior_direito.y: retorne falso
    se r2.superior_esquerdo.y <= r1.inferior_direito.y: retorne falso

    retorne verdadeiro
```

Coisas que você ainda não sabe: 

- `if` (não precisa de parêntesis na condição):

```rust
if <condição> {

}
```

- `false` e `true`: São valores `bool`. 

Então, você vai criar as duas `structs`, vai criar a função `intercepta` como um método do Retângulo, que recebe uma referência para `self` e outra referência para outra instância de `Retangulo`. 

E na função `main()` vai criar 4 retângulos: 

Dois retângulos que se interceptam:

- Retângulo 1: canto superior esquerdo (2, 6), canto inferior direito (6, 2)
- Retângulo 2: canto superior esquerdo (4, 5), canto inferior direito (8, 3)

Esses se interceptam porque ocupam a área entre x = 4 a 6 e y = 3 a 5.

Dois retângulos que não se interceptam:

- Retângulo 3: canto superior esquerdo (2, 6), canto inferior direito (6, 2)
- Retângulo 4: canto superior esquerdo (7, 5), canto inferior direito (9, 3)

E sua função `main()` terá que invocar o método `intercepta` de cada um dos primeiros retângulos (1 e 3) passando os respectivos segundos como argumento (2 e 4). Lembre-se de criar uma função para instanciar `Retangulo`. Para `Ponto` não precisa.

Fez? Não? Então joga a toalha e veja na pasta correções. 