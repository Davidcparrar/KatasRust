use std::collections::{HashMap, HashSet};

fn get_digit_variations() -> HashMap<char, Vec<char>> {
    HashMap::from([
        ('0', vec!['0', '8']),
        ('1', vec!['1', '2', '4']),
        ('2', vec!['1', '2', '3', '5']),
        ('3', vec!['2', '3', '6']),
        ('4', vec!['1', '4', '5', '7']),
        ('5', vec!['2', '4', '5', '6', '8']),
        ('6', vec!['3', '5', '6', '9']),
        ('7', vec!['4', '7', '8']),
        ('8', vec!['0', '5', '7', '8', '9']),
        ('9', vec!['6', '8', '9']),
    ])
}
fn get_pins(observed: &str) -> Vec<String> {
    let digit_variations = get_digit_variations();
    let mut possibilities: HashSet<String> = HashSet::from(["".to_string()]);

    for variated in observed.chars() {
        let variations = digit_variations
            .get(&variated)
            .expect(&format!("Unexpected character: {}", variated));

        possibilities = variations
            .iter()
            .flat_map(|v| possibilities.iter().map(move |p| format!("{}{}", p, v)))
            .collect();
    }

    possibilities.into_iter().collect()
}
// fn get_pins(observed: &str) -> Vec<String> {
//     let digit_variations = get_digit_variations();
//     let mut possibilities: HashSet<String> = HashSet::new();

//     for variated in observed.chars() {
//         let variations = digit_variations.get(&variated).unwrap();
//         let mut new_possibilities = HashSet::new();
//         for variation in variations {
//             if possibilities.is_empty() {
//                 new_possibilities.insert(format!("{}", variation));
//             } else {
//                 for possibility in &possibilities {
//                     new_possibilities.insert(format!("{}{}", possibility, variation));
//                 }
//             }
//         }
//         possibilities = new_possibilities;
//     }

//     possibilities.into_iter().collect::<Vec<String>>()
// }

fn main() {
    println!("{:?}", get_pins("8"));
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
