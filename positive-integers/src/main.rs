fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_num = input.trim().parse().unwrap();
    println!("{}", calculate_num_factors(input_num));
}

fn calculate_num_factors(mut input: u32) -> u32 {
    let mut count = 0;
    let mut i = 1;
    loop {
        if input.is_multiple_of(i) {
            eprintln!("{}", input);
            count += 1;
            input /= i;
            i = 0;
            if input == 1 {
                break;
            }
        }
        i += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(65536, 16)]
    #[case(127381, 3)]
    fn assert(#[case] input: u32, #[case] output: u32) {
        assert_eq!(calculate_num_factors(input), output)
    }
}
