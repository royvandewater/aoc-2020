use std::collections::HashMap;

pub fn round(
    max: usize,
    current: usize,
    mut cups: HashMap<usize, usize>,
) -> (usize, HashMap<usize, usize>) {
    let &first = cups.get(&current).unwrap();
    let &second = cups.get(&first).unwrap();
    let &third = cups.get(&second).unwrap();

    let picked_up = vec![first, second, third];
    let destination = get_destination(max, current, &picked_up);
    let &after_destination = cups.get(&destination).unwrap();

    cups.insert(destination, first);
    cups.insert(third, after_destination);

    let &next = cups.get(&current).unwrap();

    return (next, cups);
}

fn get_destination(max: usize, current: usize, picked_up: &Vec<usize>) -> usize {
    let mut destination = decrement(max, current);

    while picked_up.contains(&destination) {
        destination = decrement(max, destination);
    }

    destination
}

fn decrement(max: usize, current: usize) -> usize {
    match current {
        1 => max,
        _ => current - 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::{hash_map_to_vec, vec_to_hash_map};

    use super::*;

    #[test]
    fn test_example_1_round_1() {
        let current = 3;
        let cups = vec_to_hash_map(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        let (current, cups) = round(9, current, cups);

        assert_eq!(2, current);
        assert_eq!(vec![1, 5, 4, 6, 7, 3, 2, 8, 9], hash_map_to_vec(&cups));
    }
}
