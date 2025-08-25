
```rust
mod equacao {
    pub enum Raizes {
        Uma {x: f64},
        Duas {x1: f64, x2: f64},
        Nenhuma,
    }
    fn delta(a: f64, b: f64, c: f64) -> Result<f64, String> {
        let delta = b*b - 4.0*a*c;
        if delta < 0.0 {
            return Err("Delta negativo".to_string());
        }
        Ok(delta)
    }
    pub fn calcular(a: f64, b: f64, c: f64) -> Raizes {
        if let Ok(d) = delta(a,b,c) {
            if d > 0.0 {
                let x1 = (-b - d.sqrt()) / (2.0*a);
                let x2 = (-b + d.sqrt()) / (2.0*a);
                return Raizes::Duas{x1, x2};
            } else {
                let x = (-b + d.sqrt()) / (2.0*a);
                return Raizes::Uma{x};
            }
        } else {
            return Raizes::Nenhuma;
        }
    }
}

fn main() {
    // x2−5x+6=0 x1=2, x2=3
    // x2−4x+4=0 x=2
    // x2+x+1=0 nenhuma raiz
    let equacoes = vec!((1.0,-5.0,6.0), (1.0,-4.0,4.0), (1.0,1.0,1.0));
    for eq in equacoes {
        match equacao::calcular(eq.0,eq.1,eq.2) {
            equacao::Raizes::Duas{x1,x2} => println!("a: {}, b:{}, c:{} : x1={}, x2={}",eq.0,eq.1,eq.2,x1,x2),
            equacao::Raizes::Uma{x} => println!("a: {}, b:{}, c:{} : x={}",eq.0,eq.1,eq.2,x),
            equacao::Raizes::Nenhuma => println!("a: {}, b:{}, c:{} : nenhuma raiz",eq.0,eq.1,eq.2)
        }
    }
}
```