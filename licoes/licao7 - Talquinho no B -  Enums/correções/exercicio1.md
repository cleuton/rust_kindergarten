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
            0 => Moeda::UmReal,
            1 => Moeda::CinquentaCentavos,
            2 => Moeda::VinteEcincoCentavos,
            3 => Moeda::DezCentavos,
            4 => Moeda::CincoCentavos,
            5 => Moeda::UmCentavo,
            _ => Moeda::UmCentavo,
        }
    }
}

fn calcular_troco(valor_centavos: i32) -> [i32; 6] {
    let mut moedas: [i32; 6] = [0; 6];
    let mut troco = valor_centavos;

    for ix in 0..6 {
        let m = Moeda::por_indice(ix);
        let qtde = troco / m.valor();
        moedas[ix] = qtde;
        troco -= qtde * m.valor();
    }
    moedas
}

fn main() {
    let resultado = calcular_troco(187);
    for ix in 0..6 {
        let moeda = Moeda::por_indice(ix);
        let q = resultado[ix];
        if q > 0 {
            if ix == 0 {
                println!("{} moeda(s) de 1 Real", q);
            } else {
                println!("{} moeda(s) de {} centavo(s)", q, moeda.valor());
            }
        }
    }
}

```