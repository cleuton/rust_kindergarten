O exercício final é assim: 

```rust
struct Ponto {
    x: f64,
    y: f64,
}

trait Mostrar {
    fn mostrar(&self) -> String;
}

impl Mostrar for Ponto {
    fn mostrar(&self) -> String {
        format!("Ponto: (x = {}; y = {})", self.x, self.y)
    }
}

trait Figura: Mostrar {
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

impl Mostrar for Circulo {
    fn mostrar(&self) -> String {
        format!("Circulo: (centro = {}; raio = {})", self.centro.mostrar(), self.raio)
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

impl Mostrar for Retangulo {
    fn mostrar(&self) -> String {
        format!("Retangulo: (canto superior esquerdo = {}; canto inferior direito = {})", self.sup_esq.mostrar(), self.inf_dir.mostrar())
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
    println!("{}", figura1.mostrar());
    println!("{}", figura2.mostrar());
    println!("Área de r1: {} área de c1: {}", figura1.area(), figura2.area());
    println!("Perímetro r1: {} perímetro c1: {}", figura1.perimetro(), figura2.perimetro());
}
    
```