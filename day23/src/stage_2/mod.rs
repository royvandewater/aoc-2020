use std::collections::VecDeque;

mod from;
mod round;

pub struct Stage2(VecDeque<usize>);

const TEN_MILLION: usize = 10 * 1000 * 1000;

impl Stage2 {
    pub fn answer(&self) -> usize {
        let mut cups = self.0.iter().cloned().collect();

        for _ in 0..TEN_MILLION {
            println!(".");
            cups = round::round(cups);
        }

        find_product(cups)
    }
}

fn find_product(mut cups: VecDeque<usize>) -> usize {
    while cups.front().unwrap() != &1 {
        cups.rotate_left(1);
    }

    let second = cups.get(1).unwrap();
    let third = cups.get(1).unwrap();

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
