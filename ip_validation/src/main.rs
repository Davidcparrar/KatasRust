use std::{num::ParseIntError, str::FromStr};

struct Octet {
    value: u8,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseOctetError {
    InvalidInt(ParseIntError),
    EmptyInt,
    StartsWithZero,
}

impl From<ParseIntError> for ParseOctetError {
    fn from(err: ParseIntError) -> Self {
        ParseOctetError::InvalidInt(err)
    }
}

impl FromStr for Octet {
    type Err = ParseOctetError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseOctetError::EmptyInt);
        }

        if s.starts_with('0') && s.len() > 1 {
            return Err(ParseOctetError::StartsWithZero);
        }
        let value: u8 = s.parse()?;
        Ok(Octet { value })
    }
}

fn is_valid_ip(ip: &str) -> bool {
    let octets: Result<Vec<Octet>, _> = ip.split('.').map(|n| n.parse::<Octet>()).collect();
    match octets {
        Ok(vec) => vec.len() == 4,
        Err(_) => false,
    }
}

fn main() {
    is_valid_ip("1.1.1.1");
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_valid_ip;

    #[test]
    fn sample_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
    }
}
