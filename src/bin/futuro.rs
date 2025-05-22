#[derive(Debug)]
struct Futuro {
    strike: i64,
    spot: i64,
    margem: i64,
    inicial: i64,
    manutencao: i64,
    is_long_position: bool,
    tamanho_contrato: i64,
    deposito: i64,
    saque: i64,
}

impl Futuro {
    fn new(
        strike: i64,
        spot: i64,
        margem: i64,
        inicial: i64,
        manutencao: i64,
        is_long_position: bool,
        tamanho_contrato: i64,
    ) -> Self {
        Self {
            strike,
            spot,
            margem,
            inicial,
            manutencao,
            is_long_position,
            tamanho_contrato,
            deposito:0,
            saque:0,
        }
    }

    fn verifica_margem(&mut self){
        if &self.margem < &self.manutencao {
            self.deposito = &self.inicial -&self.margem;
            self.saque=0;
            println!("Aviso! Necessário depositar ${}", self.deposito);
        } else if &self.margem > &self.inicial {
            self.saque = &self.margem - self.inicial;
            self.deposito=0;
            println!("Aviso: Conta Margem comk excedente de ${}", self.saque);
        } else {
            self.saque=0;
            self.deposito=0;
        }
    }

    fn ajuste_diario(&mut self, novo_spot: i64){
        self.spot = novo_spot;
        let mut ajuste_diario: i64 = self.tamanho_contrato * (
                &self.spot - &self.strike
        );
        if !self.is_long_position {
            ajuste_diario *= -1;
        }
        self.margem += &ajuste_diario;
        self.verifica_margem();        
    }

    fn chamada_margem(&mut self, deposito: i64){
        if &deposito < &self.deposito {
            println!("Deposito minimo {}", self.deposito);
        } else {
            self.margem += deposito;
            self.verifica_margem();
        }


    }
    fn saque_excedente(&mut self, saque: i64){
        if &saque > &self.saque {
            println!("O saque não pode exceder ${}", self.saque);
        } else {
            self.margem -= saque;
            self.verifica_margem();
        }
    }
}

fn main() {
    let mut contrato = Futuro::new(
        100, 
        100, 
        1000, 
        1000, 
        800, 
        true, 
        10,
    );
    println!("{:?}", contrato);
    contrato.verifica_margem();
    contrato.ajuste_diario(105);
    contrato.saque_excedente(contrato.saque + 10);
    contrato.saque_excedente(contrato.saque);
    println!("{}", contrato.margem);
    contrato.ajuste_diario(75);
    contrato.chamada_margem(contrato.deposito - 10);
    contrato.chamada_margem(contrato.deposito);
    println!("{}", contrato.margem);

}