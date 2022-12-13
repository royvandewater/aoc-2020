use std::collections::HashMap;

pub fn vec_to_hash_map(cups_vec: &Vec<usize>) -> HashMap<usize, usize> {
    let mut cups = HashMap::new();
    for (i, &cup) in cups_vec.iter().enumerate() {
        let &next_cup = cups_vec.get(i + 1).unwrap_or(&1);
        cups.insert(cup, next_cup);
    }
    cups
}
