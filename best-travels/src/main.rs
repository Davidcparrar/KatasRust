use std::collections::BTreeMap;

fn backtract(
    ls: &mut [(i32, usize)],
    k: usize,
    current_sum: i32,
    t: i32,
    depth: usize,
    best: &mut i32,
) {
    if depth == k {
        *best = (*best).max(current_sum);
        return;
    }
    for i in 0..ls.len() {
        let (val, count) = ls[i];
        if count == 0 {
            continue;
        }
        if current_sum + val > t {
            break;
        }
        ls[i].1 -= 1;
        backtract(&mut ls[i..], k, current_sum + val, t, depth + 1, best);
        ls[i].1 += 1;
    }
}
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    if ls.len() < k as usize {
        return -1;
    }
    let mut freq = BTreeMap::new();
    for &v in ls {
        *freq.entry(v).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<(i32, usize)> = freq.into_iter().collect();

    let mut best = -1;
    backtract(&mut freq_vec, k as usize, 0, t, 0, &mut best);
    best
}

fn main() {
    let vec: Vec<i32> = (0..400).collect();
    let vec: Vec<i32> = (0..1000).flat_map(|_| vec.iter().cloned()).collect();
    let result = choose_best_sum(15, 3, &vec);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {

    use super::choose_best_sum;

    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }

    #[test]
    fn one_case() {
        let ts = &vec![50, 55];
        testing(55, 1, ts, 55);
    }
    #[test]
    fn two_case() {
        let ts = &vec![50, 55];
        testing(106, 2, ts, 105);
    }

    #[test]
    fn three_case() {
        let ts = &vec![50, 55, 56, 85];
        testing(161, 3, ts, 161);
    }

    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);
    }
}
