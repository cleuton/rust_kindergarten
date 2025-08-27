<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Olha, Mamãe: Sem as mãos! Tipos compostos 

[**Vídeo da lição**](https://youtu.be/rxw6Q_Cm84U).

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Agora o "bicho" vai pegar... O que são tipos compostos em Rust? 

Excelente pergunta! Em Rust, tipos como `String` e `&str` **não são tipos escalares** — eles pertencem a outra categoria. Tipos como `String`, `&str` e outros são chamados de: **Tipos compostos**!

Diferença entre `String` e `&str`

| Tipo     | O que é | Propriedade | Mutável? | Alocação |
|----------|---------|-------------|----------|----------|
| `String` | Tipo próprio, mutável | Dono do dado | Sim | Heap |
| `&str`   | Slice de string | Referência | Não | Stack ou estática |

Exemplos

```rust
let s1: &str = "Olá";           // string literal (estática)
let s2: String = String::from("Mundo");  // string alocada
let s3: &str = &s2;             // slice da String
```

Resumo

| Categoria | Tipos | Exemplos |
|-----------|-------|----------|
| **Scalar Types** | Tipos simples | `i32`, `f64`, `bool`, `char` |
| **Compound Types** | Tipos compostos | `String`, `&str`, `Vec<T>`, tuplas, structs |

**Memória:** Em Rust, os **tipos compostos** alocam seus valores de forma diferente dependendo do tipo específico. Tipos como `String`, `Vec<T>`, `HashMap<K, V>` e outras coleções alocam seus **dados reais no heap**, mantendo apenas metadados (como ponteiro, tamanho e capacidade) na stack. Já tipos como tuplas, structs e arrays podem alocar seus dados **inteiramente na stack** quando todos os seus campos são tipos escalares ou outros tipos stack-only. Portanto, o local de alocação depende da **estrutura interna do tipo composto**: se contém dados de tamanho dinâmico ou proprietário, usa o heap; caso contrário, pode ser totalmente stack-based.

## Exercícios

### 1 Crie um String e mostre o tamanho dele

Vamos criar um programa Rust que: 

1) Crie uma variável `String`.
2) Mostre quantos caracteres ela tem. 

Vamos lá... É muito parecido com `Java`, `Javascript` ou `Python`. Crie uma variável `nome` atribuindo um nome a ela, e use o método `.len()`! Use esse nome aqui (exatamente esse!): "José João da Silva". Quantas letras tem esse nome? `18` letras. 

Eu quero que mostre assim: `Nome: \<nome da pessoa\> tamanho: \<tamanho do nome\>`! É só usar 2 interpolações: 

`println!("v: {}, z: {}", v, z);`.

Fez? Rodou? Não deve dar erro! Se der, veja onde errou. Se "jogar a toalha", veja na pasta `correções` dessa lição. O resultado está correto? Não? Por que? 

- O nome que escolhemos tem 18 caracteres: "José João da Silva".
- O tamanho retornado pelo `.len()` tem `20` caracteres!!!

O método `String::len()` retorna **a quantidade de bytes** que a variável contém. Como Rust usa **UTF-8**, logo, caracteres fora do **ASCII** padrão podem usar mais de um byte. E temos duas letras acentuadas: `é` e `ã`! Portanto, cada um ocupa 2 bytes, daí o resultado de 20. 

Para saber a quantidade de caracteres, use o método `.count()` do método `.chars()` do `String`: `nome.chars().count()`. E agora, funcionou? Legal!

### 2 Propriedade

Em Rust uma variável é a **dona** do valor que ela contém. 

> **Ownership**: Ou "propriedade" ou "posse" em Rust é um sistema que controla **quem pode usar cada pedaço de dado** no seu programa, garantindo que **nunca haja conflitos ou erros de memória**. Basicamente, cada valor em Rust tem um único "dono", que é uma variável responsável por ele. Quando você passa esse valor para outra variável, o **controle muda de dono** (isso se chama *move*), e o dono antigo **não pode mais usá-lo**. Isso evita problemas como usar dados já liberados ou ter duas partes tentando modificar o mesmo dado ao mesmo tempo. O melhor de tudo: isso tudo é verificado **enquanto você escreve o código**, então erros de memória são pegos antes mesmo do programa rodar.

Difícil de entender, não? Isso se aplica a todo tipo de variável, mesmo os tipos escalares, mas é mais fácil perceber isso com os tipos compostos. Vamos fazer um exercício. Crie um código Rust assim: 

1) Declare uma variável String `s1` e atribua a ela um literal "Texto". 
2) Declare uma variável `s2` atribuindo a ela `s1`.
3) Use a macro `println!` para mostar o conteúdo de `s1`.
4) Use a macro `println!` para mostar o conteúdo de `s2`.

Entendeu a ordem dos comandos? Faça e rode (se "amarelar", veja na pasta `correções`). Calma que vai dar erro!

Você deve ter declarado no passo 1 a variável assim: `let s1 = "Texto";`. Quando não informamos o tipo da variável, ela assume o tipo do valor atribuído. Todo literal `String` é do tipo `&str`, portanto `s1` é um `&str` e não um `String`! Você tem duas maneiras de corrigir isso: 

- Como já mostramos, use o `String::from()`: `let s1 = String::from("Texto");`.
- Usar o método `.to_string()`: `let mut s1 = "Texto1".to_string();`. 

Agora pode tentar compilar! E DEU XABÚ!

O erro agora deve ser: `error[E0382]: borrow of moved value: s1` na linha `println!("{}", s1);`.

Vamos entender o que aconteceu... Após declarar `s1` ela tem a posse do `String` "Texto". Na linha `let s2 = s1;` estamos **MOVENDO** a posse do `String` para a variável `s2`. Depois disso, `s1` não tem mais a posse e não pode ser utilizada.

> **É diferente com tipos escalares**: Entre variáveis de tipos escalares, como: `i32`, `f64`, é feita uma **cópia** do valor quando atribuímos uma à outra. Em tipos compostos, ocorre uma **movimentação** do valor. 

Se em vez de `String` as variáveis fossem `f64` por exemplo, não daria erro algum. Apenas ele copiaria o valor de `s1` para `s2`. 


### 3 Movimento

Já vimos o que é propriedade ou posse (ownership), mas é importante saber uma coisa... Vamos fazer outro exercício, desta vez criando uma função para mostrar a quantidade de caracteres do `String`. 

1) Crie uma função `tamanho()` que receba um argumento `String` (`s: String`) e retorne um `usize` (este é o retorno do método `String::chars().count()`).
2) Essa função apenas retorna `s.chars().count()`.
3) Na função `main()` crie uma variável imutável `String` chamada `nome` contendo "Fulano de Tal". 
4) Crie uma variável `tam_nome` e invoque a função `tamanho()` passando o `nome`.
5) Faça um `println!` de `nome ` e `tam_nome`.

Compile. Vai dar erro! Deve dar um erro assim: `error[E0382]: borrow of moved value: nome` na linha `println!("{} {}", nome, tam_nome);`.

Por que? Ao invocarmos a função `tamanho()` passando a variável `nome`, ele MOVE o valor para dentro da função, ou seja, o argumento `s` agora tem a posse do `String`, que será desalocado quando a função retornar. Ao tentarmos usar `nome` novamente, ela está inválida. 

> **Por que isso não acontece com `f64`?** Porque `f64` é um tipo escalar! Entre tipos escalares ocorre uma cópia. 

### 4 Empréstimo

Você pode **emprestar** uma variável criando uma referência para ela. Esse empréstimo pode ser `imutável` ou `mutável`. Se você **emprestar** uma variável, não pode alterar seu conteúdo enquanto os empréstimos estiverem ativos (as variáveis estiverem no escopo).

Vamos tentar um exemplo simples...

>**ATENÇÃO:** Por mais que seja tentador copiar o exercício passado e só alterar, **NÃO FAÇA ISSO!** O objetivo desse curso é adquirir **FLUÊNCIA** o que só é possível se você codificar muito!

1) Crie uma função `tamanho()` que exiba o tamanho de uma **referência imutável** de `String` passada para ela (`s: &String`) e retorne `usize`.
2) O retorno dessa função deverá ser o tamanho em caracteres do string cuja referência foi passada. 
3) Na função `main()` crie uma variável `String` imutável `nome` com o conteúdo `Fulano de Tal`.
4) Crie uma variável `tam_nome` e atribua o retorno da função `tamanho()` pasando o `nome`.
5) Crie uma variável `s2` que obtém um **empréstimo imutável** da variável `nome`.
6) Faça um `println!` de `nome` e `s2`.

Se fizer direitinho, vai compilar sem erros. (Se não soube ou tem erros, veja a pasta `correções`).

### 5 Empréstimo impede alteração

Se você emprestar uma variável, ainda pode utilizá-la, mas não pode alterá-la. Vamos fazer mais um exercício: 

1) Crie uma função `main()`.
2) Crie uma variável `String` mutável `s1` com o texto: `Texto` (use `to_string()` ou `String::from()`).
3) Crie uma variável imutável `s2`, atribuindo a ela uma referência à `s1` (`let s2 = &s1;`).
4) Adicione uma exclamação à variável `s1` com o método `push_str()` (`s1.push_str("!");`).
5) Use o `println!` para mostrar `s2`.

Compile. Deu erro? Deve ter dado sim e o erro deve ser: `error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable` na linha `s1.push_str("!");`. 

Você criou uma variável mutável `s1` e poderia perfeitamente alterar seu conteúdo com `push_str()`. Porém, criou um empréstimo para `s2` e ela ainda está no escopo (é utilizada no `println!` final), assim sendo, não pode alterar seu conteúdo enquanto `s2` estiver no escopo. 

> **Otimização do compilador pode remover `s2`:** Experimente tirar ou comentar a linha final (o `println` de `s2`). Seu código vai passar só com um warning. Por que? Porque o compilador sabe que você não vai mais usar `s2`, então ele a remove do escopo e você pode alterar `s1` sem problemas. 

### 6 Só pode haver um empréstimo mutavel

Você pode ter vários empréstimos imutáveis, mas apenas um único empréstimo mutável. Se houver um empréstimo mutável, o compilador não deixará você criar mais empréstimos. 

Crie um código assim: 

1) Crie uma função `main()`.
2) Crie uma variável `String` mutável `s1` com o texto: `Texto` (use `to_string()` ou `String::from()`).
3) Crie uma variável imutável `s2`, atribuindo a ela uma referência mutável à `s1` (`let s2 = & mut s1;`).
4) Faça um `println!` da variável `s1`.
5) Crie uma variável imutável `s3`, atribuindo a ela uma referência imutável à `s1`.
6) Faça um `println!` da variável `s3`.
7) Faça um `println!` da variável `s2`.

Compile. Deu erro? Deve dar 2 erros: 

- `error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable` na linha `println!("{}",s1);`.
- `error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable` na linha `let s3 = & s1;`.

Se não deu erro ou foi diferente, veja a pasta `correções`.

Como criamos um empréstimo mutável para `s2`, a variável `s1` não pode ser mais emprestada e o `println!` de `s1` tenta fazer um empréstimo imutável dela, assim como a criação da variável `s3`. Se você quiser que isso funcione: 

- Exclua o empréstimo imutável em `s2`, retirando o `mut`; ou
- Mova o `println!` de s1 e a criação de `s3` para depois do `println!` de `s2`.

Assim, o compilador otimiza e retira `s2` do escopo. 
