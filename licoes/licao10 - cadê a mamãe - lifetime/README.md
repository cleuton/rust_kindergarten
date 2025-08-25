<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Cadê a mamãe : Lifetime

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

**Lifetimes** em Rust é uma das coisas mais **cheiradas** que existem na linguagem. Mas, se você tentar compreender a filosofia do Rust, em vez de brigar com ela, verá que faz sentido. 

Pense em lifetimes como o segurança de uma balada no modelo de memória do Rust. Todo mundo (variáveis, referências) quer entrar, dançar e se divertir. Mas o Rust não confia muito em você, então contrata um segurança durão que só deixa passar quem disser: *“até que horas vai ficar aqui dentro?”*.

Por que isso existe? Porque o Rust não tem coletor de lixo e também não te deixa jogar memória ao vento como em C. Ele precisa ter certeza de que, quando você passa uma referência, aquilo a que ela aponta ainda vai estar vivo no momento em que você usar. Sem lifetimes, você poderia acabar dançando com um fantasma, ou seja: um ponteiro solto. O Rust prefere te irritar no começo a deixar seu programa cair com um segfault no meio da festa.

A parte curiosa é que, na maioria dos casos, você nem vê os lifetimes, porque o compilador preenche tudo sozinho. É como o segurança que já conhece os clientes de carteirinha e deixa passar sem burocracia. Você só percebe que ele existe quando tenta algo esquisito demais e ele barra na porta, para salvar sua pele depois.

### Rótulos de lifetime

Em Rust, quando você escreve `'a`, `'b`, `'c` etc., isso não é string e nem variável. É só um **apelido para um tempo de vida** que o compilador precisa rastrear. Pense como se fossem etiquetas de validade em produtos do mercado. Você pode ter um leite com validade `'a` e um pão com validade `'b`. O Rust precisa saber até quando cada coisa está boa para consumo (até quando a referência é segura).

Essas etiquetas (`'a`, `'b`) não mudam nada em tempo de execução: são puramente instruções para o compilador. É como rabiscar datas de validade na lousa só para organizar a bagunça mental.

Nada como um exemplo...

```rust
fn menor<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = String::from("olá");
    let b = String::from("mundo!");

    let m = menor(&a, &b);
    println!("Menor string: {}", m);
}
```

Aqui o `'a` é a **etiqueta de validade** que garante: “o resultado da função `menor` não vai durar mais do que `s1` e `s2`”.

Se não colocássemos o `'a`, o compilador não teria como saber de quem a referência devolvida depende, e daria erro.

### Quando o compilador te dispensa de fornecer rótulos de lifetime

Agora, vem o lance do **lifetime elision** (ou “omissão do lifetime”): como essas etiquetas são meio chatas de ficar escrevendo toda hora, o Rust aplica **três regras automáticas** para você não precisar digitar `'a`, `'b` em casos óbvios.

As três regras, bem simples e com humor:

1. **Cada referência nos parâmetros ganha automaticamente a sua etiqueta**.
   Se você tem `fn foo(x: &i32, y: &i32)`, o compilador internamente imagina `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`. Ou seja, dá etiquetas diferentes para cada referência, porque não sabe se elas vivem o mesmo tempo.

2. **Se só tem uma referência de entrada, o retorno herda essa mesma etiqueta**.
   `fn bar(x: &i32) -> &i32` na real é `fn bar<'a>(x: &'a i32) -> &'a i32`. Faz sentido: só tem uma fonte, logo o retorno só pode viver tanto quanto ela.

3. **Se tem várias referências, mas uma delas é `&self` ou `&mut self`, o retorno herda a etiqueta do `self`.**
   Isso vale para métodos. O compilador pensa: “Se é um método, o resultado provavelmente depende do objeto, então uso o tempo de vida da struct dona do método”.

Resumindo: `'a`, `'b` são etiquetas de validade para referências, e o compilador já põe essas etiquetas sozinho na maioria dos casos, seguindo essas três regrinhas. Você só escreve lifetimes manualmente quando a coisa fica confusa demais e o compilador não consegue adivinhar.

Vejamos três exemplos nos quais você pode dispensar o `lifetime`: 

Boa! Vamos simplificar. O objetivo é mostrar casos em que **não precisamos escrever lifetimes** porque o compilador aplica automaticamente as regras de elision. Vou usar `&str` para manter simples.

### Exemplo 1 — Uma referência de entrada

Regra: se só existe **uma** referência na entrada, o retorno herda esse lifetime.

```rust
fn eco(s: &str) -> &str {
    s
}

fn main() {
    let palavra = String::from("rust");
    let copia = eco(&palavra);
    println!("{}", copia);
}
```

Aqui o compilador entende sozinho que o retorno vive tanto quanto `s`.

---

### Exemplo 2 — Método com `&self`

Regra: se um método recebe `&self`, o retorno herda o lifetime do `self`.

```rust
struct Mensagem {
    texto: String,
}

impl Mensagem {
    fn conteudo(&self) -> &str {
        &self.texto
    }
}

fn main() {
    let msg = Mensagem { texto: String::from("Oi!") };
    println!("{}", msg.conteudo());
}
```

Não precisamos escrever `'a`, porque o compilador sabe que o retorno depende de `self`.

---

### Exemplo 3 — Método com `&mut self`

Mesma ideia da regra 2, mas com mutabilidade explícita.

```rust
struct Contador {
    valor: i32,
}

impl Contador {
    fn valor_atual(&mut self) -> &mut i32 {
        &mut self.valor
    }
}

fn main() {
    let mut c = Contador { valor: 10 };
    let v = c.valor_atual();
    *v += 1;
    println!("{}", c.valor);
}
```

O retorno herda o lifetime de `self`, então nenhuma anotação é necessária.

---

Esses três exemplos compilam sem precisar declarar `'a`, `'b` etc., porque se encaixam nas **três regras de elision**.

## Exercícios

- Crie um exemplo com uma função que recebe três `Strings`: `s1`, `s2` e `padrao`. Se `padrao` aparecer em `s1`, devolve `s1`, se `padrão` aparecer em `s2`, devolve `s2`, caso `padrao` não apareça em nenhum dos dois, devolva `None`. 

Para essa função, você vai receber três referências para `String` e retornar uma `Option` para uma referência de `String` que deve ter o mesmo `lifetime`.

Para facilitar a sua vida, aqui está a assinatura dessa função: 

```rust
fn procure_nos_dois<'a>(s1: &'a str, s2: &'a str, padrao: &str) -> Option<&'a str> {}
```

> Em Rust, `&str` é uma visão imutável para um pedaço de texto já existente, como se fosse um rótulo que aponta para bytes UTF-8 guardados em algum lugar. Ele não é dono da memória, não pode crescer nem mudar, apenas ser lido. Já `String` é um tipo que possui e administra sua própria memória na heap, permitindo criar, expandir, modificar e liberar o texto quando sai de escopo. Em resumo, `&str` é só uma referência de leitura, enquanto `String` é uma caixa que guarda e controla o texto de verdade.
