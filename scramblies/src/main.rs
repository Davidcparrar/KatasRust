use std::collections::HashMap;

pub fn scramble(s1: &str, s2: &str) -> bool {
    let mut counts = HashMap::new();
    for c in s1.chars() {
        *counts.entry(c).or_insert(0i16) += 1;
    }
    for c in s2.chars() {
        *counts.entry(c).or_insert(0i16) -= 1;
    }
    counts.values().all(|&v| v >= 0)
}

fn main() {
    scramble("rkqodlw", "world");
}

#[cfg(test)]
mod tests {
    use super::scramble;

    fn assert_scramble(s1: &str, s2: &str, expected: bool) {
        assert_eq!(expected, scramble(s1, s2));
    }

    #[test]
    fn basic_tests() {
        assert_scramble("rkqodlw", "world", true);
        assert_scramble("cedewaraaossoqqyt", "codewars", true);
        assert_scramble("katas", "steak", false);
        assert_scramble("scriptjavx", "javascript", false);
        assert_scramble("scriptingjava", "javascript", true);
        assert_scramble("rkqodlw", "world", true);
        assert_scramble("scriptsjava", "javascript", true);
        assert_scramble("javscripts", "javascript", false);
        assert_scramble("aabbcamaomsccdd", "commas", true);
        assert_scramble("commas", "commas", true);
        assert_scramble("sammoc", "commas", true);
    }
}
