use std::io::stdin;

const LOWERCASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    println!("{}", get_missing_letters(input.trim()));
}

fn get_missing_letters(input: &str) -> String {
    let mut missing = String::new();
    for letter in LOWERCASE_ALPHABET.chars() {
        if input.find(letter).is_none() {
            missing.push(letter);
        }
    }
    if missing.is_empty() {
        "Good job!".to_string()
    } else {
        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abcdefghijklmopqrstuvwxyz", "n")]
    #[case("acdefhilkgjmmmopnttavuqyz", "brswx")]
    #[case("acegikmoqsuwybdfhjlnprtvxz", "Good job!")]
    fn assert(#[case] input: &str, #[case] output: &str) {
        assert_eq!(get_missing_letters(input), output)
    }
}
