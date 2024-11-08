fn solution(s: &str) -> Vec<String> {
    //let mut result = Vec::new();
    //let mut chars = s.chars();
    //
    //while let Some(c1) = chars.next() {
    //    let c2 = chars.next().unwrap_or('_');
    //    result.push(format!("{}{}", c1, c2));
    //}
    //
    //result
    s.chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|x| format!("{:_<2}", x.join("")))
        .collect()
}

fn main() {
    assert_eq!(solution("abc"), ["ab", "c_"]);
    assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(solution("hello"), ["he", "ll", "o_"]);
    assert_eq!(solution("a"), ["a_"]);
    assert_eq!(solution(""), [] as [&str; 0]);
}
