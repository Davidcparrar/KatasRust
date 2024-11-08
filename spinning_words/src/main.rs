const THRESHOLD: usize = 4;

fn spin_words(words: &str) -> String {
    let mut output_string = vec![];
    for word in words.split(' ') {
        let mut _word = String::from(word);
        if word.len() > THRESHOLD {
            _word = word.chars().rev().collect();
        }
        output_string.push(_word);
    }
    output_string.join(" ")
}

fn spin_words_better(words: &str) -> String {
    words
        .split(' ')
        .map(|word| {
            if word.len() >= THRESHOLD {
                word.chars().rev().collect()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(
        spin_words("You are almost to the last test"),
        "You are tsomla to the last test"
    );
    assert_eq!(
        spin_words("Just kidding there is still one more"),
        "Just gniddik ereht is llits one more"
    );
    assert_eq!(
        spin_words("Seriously this is the last one"),
        "ylsuoireS this is the last one"
    );
}
