<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Fiz dodói: Traits

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Muitas linguagens implementam o conceito de **herança** orientação a objetos, quando uma classe “filha” reaproveita atributos e métodos de uma classe “pai”, podendo também adicionar ou modificar comportamentos. É como herdar características e comportamentos de um modelo existente, evitando repetir código.

OK, **RUST NÃO TEM HERANÇA** e só relembrando: **RUST NÃO TEM CLASSES**!

Algumas linguagens possuem o conceito de **interface** ou **protocolo**, que é um contrato que define quais métodos uma classe ou objeto deve implementar, sem dizer como. Serve para garantir que diferentes implementações possam ser usadas de forma intercambiável.

Geralmente, as interfaces não armazenam estado (variáveis), mas isso pode mudar de linguagem para linguagem. 

Rust tem **trait**, normalmente traduzido como **característica** ou **traço**, mas na prática muita gente na comunidade técnica brasileira prefere manter **“trait”** sem tradução para evitar ambiguidades.

## Trait é interface?

Conceitualmente, um `trait` é parecido com uma **interface** de Java ou C#, mas com mais recursos, como implementação padrão de métodos e composição múltipla, o que torna o termo “interface” impreciso. Um trait é um conjunto de comportamentos que as structs podem implementar.

## Herança

Rust **não tem herança** de classes como Java ou C#. Uma `struct` não pode herdar nada de outra `struct`.
O que ele tem é **herança de comportamento via traits**: uma struct implementa um trait para “ganhar” métodos e contratos definidos nele, podendo usar implementações padrão ou sobrescrever. Uma struct pode implementar várias traits, mas não “herdar” campos ou estado de outra struct.

Vamos fazer como da outra vez, digitando um exemplo, compile e execute. Evite "copiar e colar"! Digite mesmo, em um bloco de notas ou GEDIT, ou mesmo Nano, salve e compile na mão! É para te ajudar na "fluência" da linguagem.

```rust
struct Ponto {
    x: f64,
    y: f64,
}

trait Figura {
    fn area(&self) -> f64 {
        0.0
    }
    fn perimetro(&self) -> f64 {
        0.0
    }
}

struct Circulo {
    centro: Ponto,
    raio: f64
}

impl Circulo {
    fn novo(centro: Ponto, raio: f64) -> Self {
        Circulo{centro, raio}
    }
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio.powi(2)
    }
    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.raio
    }
}

struct Retangulo {
    sup_esq: Ponto,
    inf_dir: Ponto,
}

impl Retangulo {
    fn novo(sup_esq: Ponto, inf_dir: Ponto) -> Self {
        Retangulo{sup_esq, inf_dir}
    }
}

impl Figura for Retangulo {
    fn area(&self) -> f64 {
        let horizontal = self.inf_dir.x - self.sup_esq.x;
        let vertical = self.sup_esq.y - self.inf_dir.y;
        horizontal * vertical
    }
    fn perimetro(&self) -> f64 {
        let horizontal = self.inf_dir.x - self.sup_esq.x;
        let vertical = self.sup_esq.y - self.inf_dir.y;
        2.0 * horizontal + 2.0 * vertical
    }
}

fn main() {
    let r1 = Retangulo::novo(
                Ponto{x: 2.0,y: 8.0}, 
                Ponto{x: 6.0,y: 2.0});
    let c1 = Circulo::novo(
                Ponto{x:2.0, y:6.0},
                5.0);
    let figura1 : &dyn Figura = &r1;
    let figura2 : &dyn Figura = &c1;
    println!("Área de r1: {} área de c1: {}", figura1.area(), figura2.area());
    println!("Perímetro r1: {} perímetro c1: {}", figura1.perimetro(), figura2.perimetro());
}
        
```

Calma! Para começar, `traits` podem ter implementações padrões (métodos com corpo), que serão invocados caso você não os sobrescreva para determinadas implementações de structs. O `trait` `Figura` tem implementação padrão para `area()` e `perimetro()` retornando 0.0 em ambos. 

Em segundo lugar, para uma `struct` implementar um `trait` tem que ter um bloco `impl` separado! O `impl` padrão é da própria `struct` e o `impl Figura for...` é a implementação do `trait` `Figura` para cada `struct`.

O **polimorfismo** é demonstrado nas linhas finais. Primeiro, criamos dois `trait objects` e apontamos para o retângulo e o círculo, e depois calculamos área e perímetro usando esses `trait objects`, que são do tipo `Figura`.

```rust
let figura1 : &dyn Figura = &r1;
let figura2 : &dyn Figura = &c1;
```

Para começar, o tipo de cada uma das variáveis `figura` tem que ser uma referência, pois não queremos MOVER o valor das variáveis originais. Em segundo lugar, o tipo de cada uma delas é um `trait` e não uma `struct`. Para que o **Rust** aceite isso, temos que informar que vamos usar `dynamic dispatch` com o `dyn`. Isso quer dizer: "Eu sei que é um `trait` mas eu vou usar os métodos do objeto concreto ao qual ele está apontando". 

Então... `figura1` e `figura2` são variáveis `trait objects`, que apontam para objetos cuja `struct` implementa `Figura`. Desta forma, posso invocar os métodos de `Figura` usando essas variáveis e os métodos dos tipos concretos serão invocados: 

```rust
println!("Área de r1: {} área de c1: {}", figura1.area(), figura2.area());
println!("Perímetro r1: {} perímetro c1: {}", figura1.perimetro(), figura2.perimetro());
```

## Exercícios

Rust tem **herança**!

- *Peraí! Não vale! Você disse que Rust não tinha herança!*

Rust não tem herança de `struct` mas tem de `trait`...

> Em traits, o que existe não é “herança” no sentido clássico de OOP, mas **subtrait**: uma trait pode exigir que outro seja implementado.

Então, vamos fazer nossa `Figura` obrigar as `structs` que a implementam também implementarem o `trait` `Mostrar`, assim como a `struct` `Ponto` de modo que possamos mostrar as variáveis no `println!` sem usar `Debug`. 

```rust
trait Mostrar {
    fn mostrar(&self) -> String;
}
...
struct Ponto {}
...
impl Mostrar for Ponto {}
...
trait Figura: Mostrar {...}
...
impl Mostrar for Retangulo {}
...
impl Mostrar for Circulo {}
...
println!("{}", figura1.mostrar());
println!("{}", figura2.mostrar());
```

Então, entendeu em linhas gerais? Você vai modificar o código que digitou para fazer isso. 

Vamos lá...

### 1 - Crie o trait `Mostrar`

Já fizemos isso, não é? A função `mostrar()` retorna um `String` formatado com as propriedades de quem implementar esse `trait`.

### 2 - Implemente `Mostrar` para `Ponto`

Tem uma `macro` chamada `format!` que funciona como o `println!`, só que retorna um `String` formatado. Use-a para exibir as propriedades do `Ponto`: 

```rust
format!("Ponto: (x = {}; y = {})", self.x, self.y)
```

### 3 - Faça `Figura` forçar a implementação de `Mostrar`

```rust
trait Figura: Mostrar {...}
```

### 4 - Implemente `Mostrar` para `Circulo`, como fez com `Ponto`

Só que `Circulo` tem `centro`, que é um `Ponto`, então basta chamar `self.centro.mostrar()`: 

```rust
format!("Circulo: (centro = {}; raio = {})", self.centro.mostrar(), self.raio)
```

### 5 - Implemente `Mostrar` para `Retangulo`

Já sabe como fazer, não? 

### Resultado esperado

```shell
Retangulo: (canto superior esquerdo = Ponto: (x = 2; y = 8); canto inferior direito = Ponto: (x = 6; y = 2))
Circulo: (centro = Ponto: (x = 2; y = 6); raio = 5)
Área de r1: 24 área de c1: 78.53981633974483
Perímetro r1: 20 perímetro c1: 31.41592653589793
```