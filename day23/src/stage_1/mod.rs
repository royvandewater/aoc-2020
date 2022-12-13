use std::collections::HashMap;

use crate::round::round;

mod from;

pub struct Stage1 {
    cups: HashMap<usize, usize>,
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

fn cups_to_string(cups: HashMap<usize, usize>) -> String {
    let mut result = "".to_string();
    let mut current = 1;

    for _ in 0..cups.len() - 1 {
        let &next = cups.get(&current).unwrap();
        result = format!("{}{}", result, next);
        current = next;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_example_1() {
        let sut = Stage1::from([3, 8, 9, 1, 2, 5, 4, 6, 7]);

        assert_eq!("67384529", sut.answer())
    }

    #[test]
    fn test_cards_to_string() {
        let sut = Stage1::from([3, 8, 9, 1, 2, 5, 4, 6, 7]);

        assert_eq!("92658374", cups_to_string(sut.cups))
    }
}
