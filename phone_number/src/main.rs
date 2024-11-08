fn create_phone_number(numbers: &[u8]) -> String {
    let numbers_str: String = numbers.iter().map(|n| n.to_string()).collect();
    format!(
        "({}) {}-{}",
        &numbers_str[0..3],
        &numbers_str[3..6],
        &numbers_str[6..10]
    )
}

fn main() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
