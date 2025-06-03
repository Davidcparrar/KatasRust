use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ParseRomanError;

impl fmt::Display for ParseRomanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid Roman numeral")
    }
}

#[derive(PartialEq, Debug)]
enum RomanLiteral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl FromStr for RomanLiteral {
    type Err = ParseRomanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseRomanError);
        }
        RomanLiteral::try_from(s.chars().next().unwrap())
    }
}

impl TryFrom<char> for RomanLiteral {
    type Error = ParseRomanError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(RomanLiteral::I),
            'V' => Ok(RomanLiteral::V),
            'X' => Ok(RomanLiteral::X),
            'L' => Ok(RomanLiteral::L),
            'C' => Ok(RomanLiteral::C),
            'D' => Ok(RomanLiteral::D),
            'M' => Ok(RomanLiteral::M),
            _ => Err(ParseRomanError),
        }
    }
}

impl RomanLiteral {
    fn parse_number(number: &str) -> Result<Vec<RomanLiteral>, ParseRomanError> {
        number.chars().map(RomanLiteral::try_from).collect()
    }

    fn value(&self) -> u64 {
        match self {
            RomanLiteral::I => 1,
            RomanLiteral::V => 5,
            RomanLiteral::X => 10,
            RomanLiteral::L => 50,
            RomanLiteral::C => 100,
            RomanLiteral::D => 500,
            RomanLiteral::M => 1000,
        }
    }
}

fn roman_as_num(roman: &str) -> u64 {
    let parsed = match RomanLiteral::parse_number(roman) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Error parsing Roman numeral, {roman}: {e}");
            return 0;
        }
    };

    let mut sum: i64 = 0;

    for i in 0..parsed.len() {
        let curr = parsed[i].value() as i64;
        let next = parsed.get(i + 1).map_or(0, |r| r.value() as i64);

        if curr < next {
            sum -= curr;
        } else {
            sum += curr;
        }
    }

    sum as u64
}

fn main() {
    let value = roman_as_num("MDCLXVI");
    println!("Decoding: {value}");
}

#[cfg(test)]
mod roman_as_num_tests {
    use super::roman_as_num;

    fn test_equal(input: &str, actual: u64, expected: u64) {
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) for the input \"{}\"",
            input
        );
    }

    #[test]
    fn basic() {
        test_equal("", roman_as_num(""), 0);
        test_equal("I", roman_as_num("I"), 1);
        test_equal("XXI", roman_as_num("XXI"), 21);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MDCLXVI", roman_as_num("MDCLXVI"), 1666);
    }

    #[test]
    fn one_through_ten() {
        test_equal("I", roman_as_num("I"), 1);
        test_equal("II", roman_as_num("II"), 2);
        test_equal("III", roman_as_num("III"), 3);
        test_equal("IV", roman_as_num("IV"), 4);
        test_equal("V", roman_as_num("V"), 5);
        test_equal("VI", roman_as_num("VI"), 6);
        test_equal("VII", roman_as_num("VII"), 7);
        test_equal("VIII", roman_as_num("VIII"), 8);
        test_equal("IX", roman_as_num("IX"), 9);
        test_equal("X", roman_as_num("X"), 10);
    }

    #[test]
    fn big_numbers() {
        test_equal("C", roman_as_num("C"), 100);
        test_equal("CDXLIV", roman_as_num("CDXLIV"), 444);
        test_equal("M", roman_as_num("M"), 1000);
        test_equal("MCMLIV", roman_as_num("MCMLIV"), 1954);
        test_equal("MCMXC", roman_as_num("MCMXC"), 1990);
        test_equal("MM", roman_as_num("MM"), 2000);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MMM", roman_as_num("MMM"), 3000);
        test_equal("MMMCM", roman_as_num("MMMCM"), 3900);
        test_equal("MMMCMXIV", roman_as_num("MMMCMXIV"), 3914);
    }
}
