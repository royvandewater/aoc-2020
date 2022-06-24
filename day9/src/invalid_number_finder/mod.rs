use std::collections::VecDeque;

mod try_from;

#[derive(Debug)]
pub struct InvalidNumberFinder {
    preamble: VecDeque<usize>,
    input: VecDeque<usize>,
}

impl InvalidNumberFinder {
    pub fn first_invalid_number(self: &mut Self) -> usize {
        loop {
            match self.step() {
                Some(opt) => return opt,
                None => continue,
            };
        }
    }

    fn step(self: &mut Self) -> Option<usize> {
        let number = self.input.pop_front().unwrap();

        if !self.is_valid(number) {
            return Some(number);
        }

        self.preamble.pop_front();
        self.preamble.push_back(number);

        None
    }

    fn is_valid(self: &Self, number: usize) -> bool {
        for (i, num1) in self.preamble.iter().enumerate() {
            let mut other = self.preamble.clone();
            other.swap_remove_front(i);

            for num2 in other {
                if num1 + num2 == number {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_two_numbers_input_valid() {
        let mut seq = InvalidNumberFinder {
            preamble: VecDeque::from([1, 2]),
            input: VecDeque::from([3]),
        };

        let res = seq.step();
        assert!(res.is_none());
        assert_eq!(VecDeque::from([2, 3]), seq.preamble);
        assert_eq!(VecDeque::from([]), seq.input);
    }

    #[test]
    fn process_two_number_preamble_second_input_is_invalid() {
        let mut seq = InvalidNumberFinder {
            preamble: VecDeque::from([1, 2]),
            input: VecDeque::from([3, 4]),
        };

        // 4 is invalid because at the time its processed, the preamble will be [2, 3]
        assert_eq!(4, seq.first_invalid_number());
    }

    #[test]
    fn process_two_number_preamble_third_input_is_invalid() {
        let mut seq = InvalidNumberFinder {
            preamble: VecDeque::from([1, 2]),
            input: VecDeque::from([3, 5, 6]),
        };

        // 6 is invalid because at the time its processed, the preamble will be [3, 5]
        assert_eq!(6, seq.first_invalid_number());
    }
}
