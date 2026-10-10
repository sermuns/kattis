fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let percentage_bet_on_option_one: u16 = input.trim().parse().unwrap();
    let (ratio1, ratio2) = calculate_switch_payout_ratios(percentage_bet_on_option_one);
    println!("{:.10}\n{:.10}", ratio1, ratio2);
}

pub fn calculate_switch_payout_ratios(percentage_bet_on_option_one: u16) -> (f32, f32) {
    let ratio_bet_on_option_one = percentage_bet_on_option_one as f32 / 100.;
    let ratio_bet_on_option_two = 1. - ratio_bet_on_option_one;

    (1. / ratio_bet_on_option_one, 1. / ratio_bet_on_option_two)
}
