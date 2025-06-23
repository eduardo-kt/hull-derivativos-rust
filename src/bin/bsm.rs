// Modelo de comportamento de preço de ação usado por Black, Scholes e Merton
// página 344 de Opções, Futuros e outros derivativos - 9ª edição
// John C Hull (2016)
// Aplicação do exemplo 15.1

struct Ativo {
    preco_inicial: f64,
    retorno_esperado: f64,
    volatilidade: f64,
}

impl Ativo {
    fn new(preco_inicial:f64, retorno_esperado:f64, volatilidade:f64) -> Ativo {
        Ativo { preco_inicial, retorno_esperado, volatilidade}
    }

    fn intervalo_preco_95(self, delta_time: f64) -> Vec<f64> {
        let mut preco95: Vec<f64> = Vec::new();
        
        let s0 = self.preco_inicial.ln();
        let mu = self.retorno_esperado;
        let sigma_squared = self.volatilidade.powf(2.0)/2.0;
        let mean = s0 + (mu - sigma_squared)* delta_time;
        
        let std_dev = self.volatilidade.powf(2.0) * delta_time;        
        

        let lower = mean - 1.96 * std_dev.sqrt();
        let upper = mean + 1.96 * std_dev.sqrt();
        
        preco95.push(lower.exp());
        preco95.push(upper.exp());
        preco95
    }
    
}

fn main() {
    let ex151 = Ativo::new(40.0, 0.16, 0.20);
    print!("{:?}", ex151.intervalo_preco_95(0.5));

}