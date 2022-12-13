use std::collections::HashMap;

pub fn vec_to_hash_map(cups_vec: &Vec<usize>) -> HashMap<usize, usize> {
    let mut cups = HashMap::new();
    for (i, &cup) in cups_vec.iter().enumerate() {
        let &next_cup = cups_vec.get(i + 1).or(cups_vec.first()).unwrap();
        cups.insert(cup, next_cup);
    }
    cups
}

pub fn hash_map_to_vec(cups: &HashMap<usize, usize>) -> Vec<usize> {
    let mut result = vec![1];
    let mut current = 1;

    for _ in 0..cups.len() - 1 {
        let &next = cups.get(&current).unwrap();
        result.push(next);
        current = next;
    }

    assert_eq!(cups.len(), result.len());

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_hash_map_simple() {
        let actual = vec_to_hash_map(&vec![1, 2, 3]);
        let expected = HashMap::from([(1, 2), (2, 3), (3, 1)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vec_to_hash_map_simple_reordered() {
        let actual = vec_to_hash_map(&vec![3, 1, 2]);
        let expected = HashMap::from([(1, 2), (2, 3), (3, 1)]);

        assert_eq!(expected, actual);
    }
}
