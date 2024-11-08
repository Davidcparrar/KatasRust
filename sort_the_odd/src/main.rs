fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = arr.iter().copied().filter(|n| n % 2 != 0).collect();
    odds.sort();

    let mut odds_iter = odds.into_iter();
    arr.iter()
        .map(|&n| {
            if n % 2 != 0 {
                odds_iter.next().unwrap()
            } else {
                n
            }
        })
        .collect()
}

fn main() {
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[]), []);
}
