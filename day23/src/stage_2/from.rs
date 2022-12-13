use std::collections::VecDeque;

use crate::input::Input;

use super::Stage2;

const ONE_MILLION: usize = 1000 * 1000;

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        let mut cups: VecDeque<usize> = input.iter().cloned().collect();

        cups.extend(10..=ONE_MILLION);

        assert_eq!(ONE_MILLION, cups.len());

        Stage2(cups)
    }
}

impl<const N: usize> From<[usize; N]> for Stage2 {
    fn from(input: [usize; N]) -> Self {
        let mut cups: VecDeque<usize> = input.iter().cloned().collect();

        cups.extend(10..=ONE_MILLION);

        assert_eq!(ONE_MILLION, cups.len());

        Stage2(cups)
    }
}
