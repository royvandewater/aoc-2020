use std::collections::HashMap;

use crate::{input::Input, vec_to_hash_map::vec_to_hash_map};

use super::Stage2;

const ONE_MILLION: usize = 1000 * 1000;

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        Stage2::from(input.iter().cloned().collect::<Vec<usize>>())
    }
}

impl<const N: usize> From<[usize; N]> for Stage2 {
    fn from(input: [usize; N]) -> Self {
        Stage2::from(Vec::from(input))
    }
}

impl From<Vec<usize>> for Stage2 {
    fn from(mut cups_vec: Vec<usize>) -> Self {
        cups_vec.extend(10..=ONE_MILLION);

        Stage2 {
            cups: vec_to_hash_map(&cups_vec),
            start: *cups_vec.first().unwrap(),
        }
    }
}
