use std::collections::{BTreeMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Position {
    First,
    Second,
    Equal,
}

impl Position {
    fn as_char(&self) -> char {
        match self {
            Position::First => '1',
            Position::Second => '2',
            Position::Equal => '=',
        }
    }
}
#[derive(Debug)]
pub struct ParsedString {
    position: u16,
    chars: BTreeMap<char, u16>,
}

impl ParsedString {
    fn new(s: &str, position: u16) -> Self {
        let mut chars = BTreeMap::new();
        for c in s.chars() {
            if c.is_alphabetic() && c.is_lowercase() {
                *chars.entry(c).or_insert(0) += 1;
            }
        }

        chars = chars.into_iter().filter(|(_, count)| *count > 1).collect();

        Self { position, chars }
    }

    pub fn get_count(&self, c: char) -> Option<u16> {
        self.chars.get(&c).copied()
    }

    pub fn get_position(&self) -> u16 {
        self.position
    }

    pub fn get_unique_chars(&self) -> Vec<char> {
        self.chars.keys().copied().collect::<Vec<_>>()
    }
}

pub fn repr(c: Vec<(u16, char, Position)>) -> String {
    c.into_iter()
        .map(|(count, c, position)| {
            format!(
                "{}:{}",
                position.as_char(),
                std::iter::repeat(c)
                    .take(count as usize)
                    .collect::<String>()
            )
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn mix(s1: &str, s2: &str) -> String {
    let parsed1 = ParsedString::new(s1, 1);
    let parsed2 = ParsedString::new(s2, 2);
    let unique_chars: HashSet<char> = parsed1
        .get_unique_chars()
        .into_iter()
        .chain(parsed2.get_unique_chars().into_iter())
        .collect();

    let mut combined_chars = unique_chars
        .into_iter()
        .filter_map(|c| {
            let count1 = parsed1.get_count(c);
            let count2 = parsed2.get_count(c);

            let (position, count) = if count1 == count2 {
                (Position::Equal, count1.unwrap_or(0))
            } else {
                let max_count = count1.unwrap_or(0).max(count2.unwrap_or(0));
                let position = if count1.unwrap_or(0) > count2.unwrap_or(0) {
                    Position::First
                } else {
                    Position::Second
                };
                (position, max_count)
            };
            Some((count, c, position))
        })
        .collect::<Vec<_>>();

    // Sort by count in descending order, then by position (First > Second > Equal) and then by character
    combined_chars
        .sort_by_key(|(count, char, position)| (std::cmp::Reverse(*count), *position, *char));

    repr(combined_chars)
}

fn main() {
    mix("eeasdhAA", "eeuiiiuoadsl");
}

#[cfg(test)]
mod tests {
    use super::mix;

    #[test]
    fn basics_mix() {
        testing(
            "Are they here",
            "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr",
        );
        testing(
            "looping is fun but dangerous",
            "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
        );
        testing(
            " In many languages",
            " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
        );
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing(
            "A generation must confront the looming ",
            "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
        );
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}
