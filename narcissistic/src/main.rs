fn narcissistic(num: u64) -> bool {
    let digits = num.to_string();
    let exponent = digits.len() as u32;

    let sum_of_powers = digits
        .chars()
        .map(|digit| match digit.to_digit(10) {
            Some(parsed) => (parsed as u64).pow(exponent),
            None => 0,
        })
        .sum::<u64>();

    sum_of_powers == num
}

fn main() {
    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(
            actual, expected,
            "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}",
            input
        )
    }

    dotest(7, true);
    dotest(371, true);
    dotest(122, false);
    dotest(4887, false);
}
