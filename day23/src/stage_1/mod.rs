use rustc_hash::FxHashMap;

use crate::{convert::hash_map_to_vec, round::round};

mod from;

pub struct Stage1 {
    cups: FxHashMap<usize, usize>,
    start: usize,
}

impl Stage1 {
    pub fn answer(&self) -> String {
        let mut cups = self.cups.clone();
        let mut current = self.start;

        for _ in 0..100 {
            (current, cups) = round(9, current, cups);
        }

        return cups_to_string(cups);
    }
}

fn cups_to_string(cups: FxHashMap<usize, usize>) -> String {
    let cups_vec = hash_map_to_vec(&cups);

    let mut result = "".to_string();

    for cup in cups_vec.iter().skip(1) {
        result = format!("{}{}", result, cup);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::convert::vec_to_hash_map;

    use super::*;

    #[test]
    fn test_answer_example_1() {
        let sut = Stage1::from([3, 8, 9, 1, 2, 5, 4, 6, 7]);

        assert_eq!("67384529", sut.answer())
    }

    #[test]
    fn test_cards_to_string_example_1() {
        let input = vec_to_hash_map(&vec![5, 8, 3, 7, 4, 1, 9, 2, 6]);

        assert_eq!("92658374", cups_to_string(input))
    }

    #[test]
    fn test_cards_to_string_simple() {
        let input = vec_to_hash_map(&vec![1, 2, 3]);

        assert_eq!("23", cups_to_string(input))
    }

    #[test]
    fn test_cards_to_string_simple_reordered() {
        let input = vec_to_hash_map(&vec![3, 1, 2]);

        assert_eq!("23", cups_to_string(input))
    }
}
