use num::bigint::BigInt;
use num::traits::{One, Zero};
use std::ops::Neg;

pub fn fib(n: i32) -> BigInt {
    let negative = n < 0;
    let n = n.unsigned_abs() as usize;

    let (f, _) = fast_doubling(n);

    if negative && n % 2 == 0 { f.neg() } else { f }
}

// Returns (F(n), F(n+1))
fn fast_doubling(n: usize) -> (BigInt, BigInt) {
    if n == 0 {
        return (BigInt::zero(), BigInt::one());
    }

    let (fk, fk1) = fast_doubling(n / 2);

    // F(2k) = F(k) * (2*F(k+1) - F(k))
    let f2k = &fk * (2 * &fk1 - &fk);
    // F(2k+1) = F(k)^2 + F(k+1)^2
    let f2k1 = &fk * &fk + &fk1 * &fk1;

    if n % 2 == 0 {
        (f2k, f2k1)
    } else {
        (f2k1.clone(), f2k + f2k1)
    }
}
fn main() {
    println!("{}", fib(29138139));
}

#[cfg(test)]
mod sample_tests {
    use super::fib;
    use num::bigint::BigInt;
    use num::traits::{One, Zero};
    use std::str::FromStr;

    fn dotest(n: i32, expected: BigInt) {
        let actual = fib(n);
        assert!(
            actual == expected,
            "Test failed with n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\""
        )
    }

    #[test]
    fn small_positive_numbers() {
        dotest(0, BigInt::zero());
        dotest(1, BigInt::one());
        dotest(2, BigInt::one());
        dotest(3, BigInt::from(2));
        dotest(4, BigInt::from(3));
        dotest(5, BigInt::from(5));
    }

    #[test]
    fn small_negative_numbers() {
        dotest(-1, BigInt::from(1));
        dotest(-6, BigInt::from(-8));
        dotest(-96, BigInt::from_str("-51680708854858323072").unwrap());
    }

    #[test]
    fn large_numbers() {
        dotest(
            -500,
            BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
            .unwrap()
        );

        dotest(
            1000,
            BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
            .unwrap()
        );
    }
}
