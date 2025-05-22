use std::fmt;

#[derive(Debug)]
struct Termo {
    strike_price: u64,
    spot_price: u64,
}

impl Termo {
    fn new(strike_price: u64, spot_price: u64) -> Self {
        Self { strike_price, spot_price }
    }
}

impl fmt::Display for Termo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Termo(K:${:.2}, S:${:.2})",
            self.strike_price as f64 / 100.0,
            self.spot_price as f64 / 100.0,
        )
    }
    
}

fn main() {
    let termo = Termo::new(1300,1645);
    println!("{:?}", termo);
    println!("{}", termo);
}
