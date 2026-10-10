pub fn calculate_switch_payout_ratios(percentage_bet_on_option_one: u16) -> (f32, f32) {
    let ratio_bet_on_option_one = percentage_bet_on_option_one as f32 / 100.;
    let ratio_bet_on_option_two = 1. - ratio_bet_on_option_one;

    (1. / ratio_bet_on_option_one, 1. / ratio_bet_on_option_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn within_tolerance(x: f32) -> bool {
        x.abs() < 1e-3
    }

    #[test]
    fn sample1() {
        let (ratio1, ratio2) = calculate_switch_payout_ratios(80);
        assert!(within_tolerance(ratio1 - 1.25));
        assert!(within_tolerance(ratio2 - 5.));
    }

    #[test]
    fn sample2() {
        let (ratio1, ratio2) = calculate_switch_payout_ratios(15);
        assert!(within_tolerance(ratio1 - 6.6666666667));
        assert!(within_tolerance(ratio2 - 1.1764705882));
    }
}
