Eis a correção: 

```rust
enum Moeda {
    UmCentavo,
    CincoCentavos,
    DezCentavos,
    VinteEcincoCentavos,
    CinquentaCentavos,
    UmReal
}

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
}

fn calcular_troco(valor_centavos: i32) -> [i32;6] {
    let mut moedas : [i32;6] = [0;6];
    let mut troco = valor_centavos;
    // Calcular o número de cada moeda
    for i in 0..6 {
        if troco >= moedas[i] {
            let ix = 5 - i; 
            let moeda = Moeda::por_indice(ix);
            let qtde = troco / moeda.valor(); 
            moedas[ix] = qtde;
            troco = troco - (qtde * moeda.valor());
        }
    }
    moedas
}

fn main() {
    let resultado = calcular_troco(187);
    for i in 0..6 {
        let ix = 5 - i;
        let moeda = Moeda::por_indice(ix);
        if i == 0 {
            if resultado[0] > 0 {
                println!("{} moeda(s) de 1 Real",resultado[ix]);
            }
        } else {
            if resultado[ix] > 0 {
                println!("{} moeda(s) de {} centavo(s)",
                        resultado[ix], moeda.valor());
            }
        }
    }
}
```