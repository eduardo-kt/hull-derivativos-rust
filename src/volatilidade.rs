// implementa exemplo 15.4 (p. 350)
pub fn ex154() {

    // sequência de preços de ações durante 21 dias de negociação consecutivos
    let dados:Vec<f32> = vec![
        20.00, 20.10, 19.90, 20.00, 20.50,
        20.25, 20.90, 20.90, 20.90, 20.75,
        20.75, 21.00, 21.10, 20.90, 20.90,
        21.25, 21.40, 21.40, 21.25, 21.75, 22.00];

    let mut ui:Vec<f32> = Vec::new();
    // calcula retorno diário
    for janela in dados.windows(2) {
        let anterior = janela[0];
        let atual = janela[1];
        let resultado = (atual/anterior).ln();
        ui.push(resultado);
    }

    // soma dos retornos diários
    let sum_ui: f32 = ui.iter().sum();    
    // soma dos quadrados
    let sum_squares: f32 = ui.iter().map(|x| x.powi(2)).sum();
    // número de dias (n) = 20
    let n = dados.len() as f32 -1.0;

    let s = ((sum_squares/(n-1.0)) - (sum_ui.powi(2)/(n*(n-1.0)))).sqrt();
    let delta_t = 252;  // 1 ano ~ 252 dias
    let tau: f32 = 1.0/delta_t as f32;  // intervalo de tempo em anos

    // volatilidade anual estimada
    let sigma_hat = s/tau.sqrt();
    let stde = sigma_hat/(2.0*(n as f32)).sqrt();

    println!("/
    Soma dos Ui: {:.5}
    Soma dos Ui²: {:.5}
    Número de observações (n): {}
    Estimativa do desvio padrão de ui (s) {:.5}
    Estimativa do desvio padrão (sigma-hat) {:.5}
    Erro padrão do sigma-hat: {:.5}",
    sum_ui, sum_squares, n, s, sigma_hat, stde);

}

// sigma_dia: volatilidade por dia de negociação
pub fn volatilidade_ano(sigma_dia: f32, dias_maturity: u32) -> f32 {
    let dias_ano = 252.0 /(dias_maturity as f32);
    sigma_dia * dias_ano.sqrt() * 100.0
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volatilidade_ano() {
        assert_eq!(volatilidade_ano(0.064, 126), 9.050967);
    }
}