<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Onde a criança chora e a mãe não vê: Arrays, slices e vec 

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

É claro que Rust tem variáveis multivaloradas: 

- De tamanho fixo, como arrays: `let v = [4.0, 7.0, 10.0, 1.0, 3.0];`.
- De tamanho variável, como vecs: `let v = vec![4.0, 7.0, 10.0, 1.0, 3.0];`. Usamos a macro `vec!`. 

E existem os slices, como: `let s = &v;`.

> **Slice** é uma referência a uma sequência contígua de elementos em memória que permite acessar parte ou todos os elementos de um array, vetor ou string sem tomar posse dos dados. Ele funciona com arrays `&arr[..]`, vectors `&vec[..]` e strings `&str[..]`, criando uma "janela" para visualizar dados contíguos na memória de forma segura e eficiente. 

## Exercícios

### 1 Crie um array fixo de `f64` e calcule a média dos números

Vamos criar um programa Rust que: 

1) Crie um array fixo de `f64` com os números: 4.0, 7.0, 10.0, 1.0, 3.0.
2) Crie uma variável `soma` `f64` para acumular a soma dos valores.
3) Itere sobre o array acumulando cada elemento em `soma`.
4) Faça um `println!` para mostrar o resultado da soma dividido pela quantidade de elementos (lembre-se de fazer cast no divisor). 

Tá... Já sei de um primeiro problema: Como usar um `for` em Rust para navegar no array? Em Javascript seria algo assim: 

```javascript
let v = [4.0, 7.0, 10.0, 1.0, 3.0];
let soma = 0.0;

for (let x = 0; x < v.length; x++) {
    soma += v[x];
}

console.log("A média é " + (soma / v.length));
```

Em Rust o `for` não tem parêntesis e não precisamos usar essas expressões, podendo usar um range: `for x in 0..v.len() {}` onde `v` é o nome da variável do array e `len()` é um método que retorna a quantidade de elementos. O Range é sempre aberto no final, então vai de zero até `tamanho - 1`. E usamos a indexação como no Javascript. 

Se quiser algo diferente, pode usar a variação que pega apenas o valor dos elementos, mas não o seu índice: `for x in v {}`.

Fez? Compilou? Rodou? A média foi: `5`? Caso contrário, consulte as `correções`.

### 2 Crie um Vec e calcule a média dos elementos

Agora, vamos criar um Vec com os elementos: 4.0, 7.0, 10.0, 1.0, 3.0 e calcular a média. O `Vec` você já sabe criar (mostrei no início a macro `vec!`). Para navegar, use o `for x in &v {}` sem índice (note que precisei usar uma referência à `v`).

> **Por que usar referência com Vecs:** `Vec` é um tipo composto. Já falamos que tipos compostos, quando enviados a funções ou atribuídos à variáveis, são MOVIDOS! Se você não usar uma referência, ele vai mover o Vec para a iteração do `for` e depois, quando precisar usar o `v`, vai dar erro. 

> **Arrays fixos são copiados e não movidos:** Sim, se o tipo de dados dos elementos for um tipo escalar. Se for um array de tipo composto, então o array também será movido. 

**Desta vez não tem passo a passo**.

Compilou? Não? Rodou e mostrou média: `5`? Não? Veja as `correções`.

**E se eu quiser pegar o índice também?** Bom, aí você tem que usar um `iterator` através do método: `.iter().enumerate()`: 

```rust
    for (i, &valor) in v.iter().enumerate() {
        println!("Índice: {}, Valor: {}", i, valor);
        soma += valor;
    }    
```

> **Porque `&valor` no `for`?** Porque o método iter() retorna um iterador sobre referências aos elementos, não os elementos diretamente.

### 3 Agora faça com função

Crie um código que calcule a média de um array fixo com os números: 4.0, 7.0, 10.0, 1.0, 3.0, utilizando uma função `calcular()` que retorne o valor da média. 

Claro que temos problemas aqui! Como declarar um array fixo como um argumento para a função? Em Rust, a forma de declarar um array pode conter seu tipo também (que inclui a quantidade de posições). Se for um array fixo, temos que informar a quantidade de posições. Normalmente, não informamos o tipo do array porque o compilador consegue inferir a partir dos elementos iniciais que fornecemos: 

`let v = [4.0, 7.0, 10.0, 1.0, 3.0];`

Mas podemos declar com o tipo do array, contendo o tipo dos elementos e a quantidade deles: 

`let v: [f64;5] = [4.0, 7.0, 10.0, 1.0, 3.0];`

Então, o tipo desse array é `[f64;5]`, o que você vai utilizar na declaração da função. 

Como sempre, ao dividir pelo tamanho do array, não se esqueça de fazer o `cast`: `soma / numeros.len() as f64`.

Deu certo? Ok.

### 4 Agora use um Vec com função

Crie um código que calcule a média de um Vec com os números: 4.0, 7.0, 10.0, 1.0, 3.0, utilizando uma função `calcular()` que retorne o valor da média. 

Como declaramos o tipo de um Vec? Bom, o tipo você já sabe: `Vec`, só que ele pode ser um Vec de qualquer coisa, então é comum usarmos a sintaxe de tipos genéricos: `Vec<f64>`. Lembre-se: 

- Array de tamanho fixo, cujo tipo dos elementos é escalar, é sempre copiado. 
- Vec é tipo composto, sendo que é sempre movido!

O que isso te lembra? Como você passa um String para uma função? Sim: Por referência!

Faça o código, compile e execute. Deve dar média 5.

> **Desreferenciação::** Ao navegar na referência de um Vec, não é necessário desreferenciar a variável para somar. Mas para atribuir sim. 

> Neste caso em especial não precisaríamos passar uma referência ao Vec para a função calcular, deixando que ela mova o vec para `numeros`, pois não usamos mais o `v` na função `main()` depois de invocar `calcular()`. Mas esta é uma prática ruim.

### 5 Usando um slice com função 

Crie um código que calcule a média de um array fixo com os números: 4.0, 7.0, 10.0, 1.0, 3.0, utilizando uma função `calcular()` que receba um slice do array e retorne o valor da média. 

Como declaramos um slice de array em um argumento de função? `fn calcular(numeros: &[f64]) -> f64 {};`. Um slice de array deve ser declarado como se fosse uma referência para um array, passando o tipo. Isso serve para arrays fixos e Vecs. 

Ao retornar a média, omita o comando `return`! O Rust sempre retorna a última expressão, desde que não termine com ";": 

`soma / numeros.len() as f64`

Sem parêntesis e sem ponto-e-vírgula. 



