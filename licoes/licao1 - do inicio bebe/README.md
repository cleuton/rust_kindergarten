<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Do início, bebê

[**Vídeo da lição**](https://youtu.be/4Pp4zsnziDU).

Para começar, [**instale o Rust!**](https://www.mycompiler.io/pt/new/rust). 

A estrutura básica de um programa **Rust** é exatamente essa: 

```Rust
fn main() {
    println!("Hello world!")
}
```

Eu quero que você memorize esse trecho de código. E entenda! O que ele faz? 

1) Declara uma função `main()` que não recebe argumentos e nem retorna valor algum. 
2) No corpo da função, usa a **macro** `println!` para mostrar a mensagem "Hello world!" e pular uma linha.

> **Macro:** Uma macro em Rust é como um atalho que o compilador usa para gerar código antes de compilar de verdade. Quando você escreve o `println!` (observe o `!`, que indica “isto não é uma função comum, mas uma macro”) é expandido pelo compilador numa série de instruções que formatam a string e chamam funções internas para imprimir no console. Quem aprende Rust não precisa decorar tudo de uma vez: basta lembrar que toda vez que vir algo terminando em `!`, foi ele quem escreveu o código extra para você antes de rodar o programa.

Orientações para fazer os exercícios: 

1) Abra um **bloco de notas** ou um **gedit**, **nano**, ou seja: Um editor sem IA e sem auto-complete.
2) Abra um CMD ou terminal e vá para a pasta onde pretende criar os scripts Rust.
3) Crie seus scripts com esse nome: `exercicion.rs` onde `n` é o número do exercício. 
4) Compile com: `rustc -o exercicion exercicion.rs`.
5) Execute com: `./exercicion` (Linux, MacOS) ou `exercicion` (Windows).

## Exercícios

### 1 Calcule a média

Quero que você imprima a média dos números: 5, 7, 3, 1, 2, 10, 6.

Não adianta fazer cara de terror! Você sabe calcular a média, não? Crie uma expressão e mostre-a! Você já sabe como fazer isso. Se não sabe, senta e chora e depois veja nas correções (exercício 1).

Tome coragem e encare, vai tomar erro mesmo e é para aprender. 

**Placeholders no println**

Se você fez algo assim: `println!((5+7+3+1+2+10+6)/7)` vai dar erro, pois a macro `println!` espera um literal string. E agora? Se você tem esperiência com outras linguagens, sabe alguns atalhos (que talvez não funcionem em Rust). Mas tem um jeito utilizando `placeholder`, que é interpolar um texto com variáveis: 

```rust
let nome = "Fulano";
println!("Bom dia, {}", nome);
```

Ele vai substituir o `placeholder` "{}" pela variável que está depois da vírgula e converter tudo para um string. 

**Divisão de inteiros**

Então, se você fizer isso aqui ó: `println!("{}",(5+7+3+1+2+10+6)/7)` vai obter a resposta: **4**. Mas se tiver o trabalho de calcular na mão, verá que o resultado correto é: **4,857...**. Houve um truncamento aí! Ele está fazendo a divisão de inteiros por um inteiro. Mas podemos fazer como no **Java** e colocar um ponto decimal no divisor? 

Se você tentar isso: `println!("{}",(5+7+3+1+2+10+6)/7.0)` vai tomar um erro de compilação: cannot divide `{integer}` by `{float}`! 

Os números da soma que é o dividendo são todos inteiros, e o divisor é um número real (f64 - Ponto flutuante de 64 bits, se o tipo não foi informado). O compilador não vai aceitar isso. Saídas? Claro que sim: 

1) Tentar colocar ponto decimal em uma das parcelas do dividendo, como fazemos em Java. Vai dar erro: O compilador não vai aceitar somar um f64 com um i32 (literais inteiros possuem tipo inteiro de 32 bits por default). 
2) Colocar ponto decimal em todas as parcelas do dividendo? Vai funcionar! Mas fica muito verboso: `println!("{}",(5.0+7.0+3.0+1.0+2.0+10.0+6.0)/7.0);`

> **Tipos escalares:** Em Rust, tipos numéricos (`i32`, `i64`, `f32`, `f64`, `bool`, `char`) são chamados assim. Eles alocam a memória na `pilha` da função atual e nunca no `heap`. São variáveis **automáticas**. 

**Cast de elementos**

Rust possui `cast` de variáveis e operandos. A sintaxe é: `\<variável / literal\> as \<novo tipo\>`. Já sabemos que o `7.0` é `f64`, portanto podemos fazer assim: `println!("{}",(5+7+3+1+2+10+6) as f64 /7.0)`. Isso vai funcionar. 


### 2 Calcule a média das notas

Agora, crie 4 variáveis: `nota1`, `nota2`, `nota3` e `media` (não use acentuação). Para as notas use: 5.0, 7.0 e 8.0 e para a média, calcule o resultado. Mostre a mensagem: "A média é: xx" com o valor da média. 

**O que sabemos?**

- Sabemos que a expressão pode ser: `(nota1 + nota2 + nota3) / 3.0`. As variáveis `notaX` devem ter o tipo `f64`.
- Sabemos interpolar variáveis com texto no `println`.

Como declarar variáveis em **Rust**? Use o comando `let`, como no `javascript`! Exatamente como no Javascript!

```javascript
// Javascript:
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media = (nota1+nota2+nota3)/3.0;
```

Pronto! Agora crie o código como eu mostrei: 

- `nano exercicio2.rs`
- `rustc -o exercicio2 exercicio2.rs`
- `./exercicio2` ou `exercicio2`

Deve funcionar sem erro. Se tiver dúvida veja na pasta `correções`. 

**Imutabilidade**

Agora, modifique o exercício para adicionar 1 à variável média antes de mostrar o resultado: 

```rust
let media = (nota1+nota2+nota3)/3.0;
media = media + 1.0;
```

Compile. O que aconteceu? Pode explicar? Deve ter retornado esse erro: 

`error[E0384]: cannot assign twice to immutable variable media`

Em Rust todas as variáveis são **imutáveis**. Você pode atribuir um valor mas, depois que fez isso, não pode mudar esse valor. Por exemplo, esse código seria válido: 

```rust
fn main() {
   let nota1 = 5.0;
   let nota2 = 7.0;
   let nota3 = 8.0;
   let media: f64; 
    media = (nota1+nota2+nota3)/3.0;
   println!("A média é: {}",media);
}
```

Aqui você viu a forma de declaração de variáveis com tipo (`let media: f64;`). Guarde bem essa forma que vale para tudo. Mas só declaramos a variável `media` sem inicializá-la. O compilador permite atribuírmos um valor a ela uma única vez. 

**Mutabilidade**

Mas e se quisermos alterar o valor de uma variável? Daí o nome: `variável`! Neste caso, basta acrescentar a palavra-chave `mut` após o `let`. Modifique isso no código e tente rodar:

```rust
let mut media = (nota1+nota2+nota3)/3.0;
media = media + 1.0;
```

Agora, a variável `media` é mutável! 

### 3 Use uma função

Agora, vamos criar uma função para calcular e retornar a média, e vamos atribuir o resultado dessa função à variável `media`. Precisamos criar uma função que receba as três variáveis e retorne um valor real. Como criamos funções em Rust? Isso você já sabe, pois toda hora cria a função `main()`. Tente fazer como em Javascript...

```javascript
function calcular(n1, n2, n3) {
    return (n1 + n2 + n3) / 3.0;
}

let nota1 = 5.0;
let nota2 = 7.0;
let nota3 = 8.0;
let media = calcular(nota1, nota2, nota3);
```

Bom, a assinatura da função deve começar com `fn` em vez de `function` (sinceramente, acho a maior b...) isso nós sabemos. E os argumentos? Será que o compilador não vai reclamar? Vai sim, já que Rust é uma linguagem de tipagem forte. Já vimos como declaramos variáveis com tipo e precisamos fazer isso: `fn calcular(n1: f64, n2: f64, n3: f64) {}`. Tente fazer e compilar. 

Você deve ter tomado um erro de `mismatched types` no comando `return` da função `calcular()`. Você não informou o tipo de dados que a função retornaria e depois está tentando retornar um valor (que seria `f64`). Como declarar o tipo de dados de retorno da função? Vejamos isso em duas linguagens diferentes: 

```java
public class ****** {
    public double calcular(double n1, double n2, double n3) {
        return (n1 + n2 + n3) / 3.0;
    }

```

```go
func calcular(n1, n2, n3 float64) float64 {
    return (n1 + n2 + n3) / 3.0
}
```

Em Rust é parecido com **Python** quando usamos `type hints`: 

```python
def calcular(n1: float, n2: float, n3: float) -> float:
    return (n1 + n2 + n3) / 3.0
```

Só que o tipo é `f64`. Tente novamente! Agora funcionou, certo? 

### 4 Referência

Agora, façamos algo diferente. Vamos criar uma função que calcula a média e a coloca na mesma variável média, sem retornar nada. Você vai precisar criar mais um argumento a variável `m` e retirar o tipo de retorno e o comando `return`: 

`fn calcular(n1: f64, n2: f64, n3: f64, m: f64) {}`

Atribua à própria variável `m` o resultado. 

E na função `main()` apenas invoque a função `calcular` obter o valor de retorno. Vai ter que declarar `media` na função `main()` como `mut` novamente.

Você deve ter tomado alguns warnings, e talvez esse erro: `error[E0384]: cannot assign to immutable argument m`. Você declarou a variável `media` como `mut`, faltou declarar o argumento `m` como `mut`. Isso pode ser feito assim: `mut m: f64` na assinatura da função. 

Tente agora. Funcionou? Sim, mas além dos warnings deve ter dado média zero! Como pode? Em Rust, não importa se a variável original e o argumento são `mut` ele sempre passa uma **cópia** do valor para a função! 

> **Atenção:** No caso de variáveis simples, primitivas, como as variáveis numéricas, ele sempre passa a cópia da variável. Em outros casos é diferente!

Para que essa versão funcione, você precisa invocar a função passando uma **referência mutável** para a variável `media`! Referências em Rust são denotadas por um caracter `&` ("e" comercial): `calcular(nota1, nota2, nota3, & mut media);`. Tente agora. Vai dar outro erro!

`mismatched types`. Por que? Vamos pensar... 

1) As variáveis primitivas (como `f64`) sempre são passadas por valor. 
2) Nós estamos passando uma **referência mutável** para uma variável `f64` mas o tipo do argumento `m` é `mut m f:64`.
3) São tipos diferentes!

Podemos declarar da mesma maneira que estamos chamando: `m: & mut f64`. Agora sim! Estamos declarando que recebemos uma referência mutável para um `f64`! Tente agora!

Vai dar erro novamente! Você já deve estar de saco cheio, não? Calma que vai piorar muito!

Você passou uma referência, portanto, na função `calcular()` o argumento `m` é uma referência mutável para uma variável `f64`. Fazendo uma analogia muito ruim, `m` é como se fosse um ponteiro para a variável (não é verdade, é só para você entender). E você não pode atribuir valor a um ponteiro, mas ao conteúdo ao qual ele aponta. 

Isso se chama `dereferencing` ou `desreferenciação` é uma operação para acessar o conteúdo apontado pela referência e não a própria referência (o endereço da variável). Fazemos isso colocando um `*` antes de atribuir ao argumento `m`: `*m = (n1 + n2 + n3) / 3.0;`.

Vamos resumir o que tivemos que fazer nesse exercício: 

1) Elimitar o tipo de retorno e mudar a assinatura da função `calcular()` para receber uma referência mutável a uma variável `f64`: 

```rust
fn calcular(n1: f64, n2: f64, n3: f64, m: & mut f64) {
```
2) Eliminar o `return` da função `calcular()`.

3) Declarar `media` na função `main()` como mutável: 

```rust
let mut media = 0.0;
```

4) Passar uma referência mutável ao invocar a função `calcular()` na função `main()`:

```rust
calcular(nota1, nota2, nota3, & mut media);
```

5) Dentro da função `calcular()` desreferenciar o atributo `m` e atribuir ao valor apontado o resultado da média: 

```rust
*m = (n1 + n2 + n3) / 3.0;
``` 

Pronto! Usamos uma referência. Isso é muito **feio**: Funções que alteram valores de parâmetros! Mas foi para explicar referências para você. 

## Recaptulando

Se você fez TODOS os exercícios, sem copiar de nada e sem usar auto-complete, então você viu: 

- Como criar uma função `main()`, compilar e executar código Rust.
- O que são tipos escalares e os principais tipos numéricos. 
- Como usar interpolação para concatenar strings e variáveis para a macro `println`.
- Como fazer cast de variáveis. 
- Imutabilidade e mutabilidade.
- Como criar e usar funções que recebem e retornam valores.
- O que são referências e como passar uma referência para uma função. 


