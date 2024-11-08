use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    let mut counter: HashMap<char, u16> = HashMap::new();
    let word = word.to_lowercase();

    word.chars().for_each(|c| {
        counter
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    });

    word.chars()
        .map(|c| match counter.get(&c) {
            Some(&count) if count > 0 => ")".to_string(),
            _ => "(".to_string(),
        })
        .collect()
}

fn main() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
