fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if a < 0 || b < 0 || c < 0 {
        return false;
    };

    !((a + b <= c) || (b + c <= a) || (c + a <= b))
}

fn main() {
    assert_eq!(is_triangle(1, 2, 2), true);
    assert_eq!(is_triangle(7, 2, 2), false);
    assert_eq!(is_triangle(1, 2, 3), false);
    assert_eq!(is_triangle(1, 3, 2), false);
    assert_eq!(is_triangle(3, 1, 2), false);
    assert_eq!(is_triangle(5, 1, 2), false);
    assert_eq!(is_triangle(1, 2, 5), false);
    assert_eq!(is_triangle(2, 5, 1), false);
    assert_eq!(is_triangle(4, 2, 3), true);
    assert_eq!(is_triangle(5, 1, 5), true);
    assert_eq!(is_triangle(2, 2, 2), true);
    assert_eq!(is_triangle(-1, 2, 3), false);
    assert_eq!(is_triangle(1, -2, 3), false);
    assert_eq!(is_triangle(1, 2, -3), false);
    assert_eq!(is_triangle(0, 2, 3), false);
}
