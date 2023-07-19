impl Carro {
    fn acelerar(&self) {
        println!("{} está acelerando!", self.modelo);
    }

    fn pintar(&mut self, nova_cor: String) {
        self.cor = nova_cor;
    }

}

struct Carro {
    modelo: String,
    cor: String,
    valor: f64
}

fn main() {
    let mut chevrolet = Carro{
        modelo: String::from("Celta"),
        cor: String::from("Preto"),
        valor: 10_000.00,
    };

    println!("Modelo >> {}\nPreço >> {}\nValor >> {}", chevrolet.modelo, chevrolet.cor, chevrolet.valor);
    
    chevrolet.acelerar();
    chevrolet.pintar(String::from("Amarelo"));
    
    println!("Modelo >> {}\nPreço >> {}\nValor >> {}", chevrolet.modelo, chevrolet.cor, chevrolet.valor);

}
