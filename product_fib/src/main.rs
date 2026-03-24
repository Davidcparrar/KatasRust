fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;
    while a * b < prod {
        let temp = a;
        a = b;
        b = temp + b;
    }
    (a, b, a * b == prod)
}

fn main() {
    println!("{:?}", product_fib(4895));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
