O exercício de interceptação de Retângulos é assim: 

```rust
struct Ponto {
    x: f64,
    y: f64,
}

struct Retangulo {
    sup_esq: Ponto,
    inf_dir: Ponto,
}

impl Retangulo {
    fn novo(sup_esq: Ponto, inf_dir: Ponto) -> Self {
        Retangulo{sup_esq, inf_dir}
    }
    fn intercepta(&self, outro: &Retangulo) -> bool {
        if self.inf_dir.x <= outro.sup_esq.x {
            return false;
        }
        if outro.inf_dir.x <= self.sup_esq.x {
            return false;
        }
        if self.sup_esq.y <= outro.inf_dir.y {
            return false;
        }
        if outro.sup_esq.y <= self.inf_dir.y {
            return false;
        }
        true
    }
}

fn main() {
    let r1 = Retangulo::novo(
                Ponto{x: 2.0,y: 6.0}, 
                Ponto{x: 6.0,y: 2.0});
    let r2 = Retangulo::novo(
                Ponto{x: 4.0,y: 5.0}, 
                Ponto{x: 8.0,y: 3.0});
    let r3 = Retangulo::novo(
                Ponto{x: 2.0,y: 6.0}, 
                Ponto{x: 6.0,y: 2.0});
    let r4 = Retangulo::novo(
                Ponto{x: 7.0,y: 5.0}, 
                Ponto{x: 9.0,y: 3.0});
    let inter_r1_r2 = r1.intercepta(&r2);
    let inter_r3_r4 = r3.intercepta(&r4);
    println!("r1 intercepta r2: {}", inter_r1_r2);
    println!("r3 intercepta r4: {}", inter_r3_r4);
}
```