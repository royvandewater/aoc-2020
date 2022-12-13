use std::collections::HashMap;

use crate::round::round;

mod from;

pub struct Stage2 {
    cups: HashMap<usize, usize>,
    start: usize,
}

const ONE_MILLION: usize = 1000 * 1000;
const TEN_MILLION: usize = 10 * ONE_MILLION;

impl Stage2 {
    pub fn answer(&self) -> usize {
        let mut cups = self.cups.clone();
        let mut current = self.start;

        for _ in 0..TEN_MILLION {
            (current, cups) = round(ONE_MILLION, current, cups);
        }

        find_product(cups)
    }
}

fn find_product(cups: HashMap<usize, usize>) -> usize {
    let second = cups.get(&1).unwrap();
    let third = cups.get(second).unwrap();

    second * third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_example_1() {
        let sut = Stage2::from([3, 8, 9, 1, 2, 5, 4, 6, 7]);

        assert_eq!(149245887792, sut.answer())
    }
}
