use crate::{convert::vec_to_hash_map, input::Input};

use super::Stage1;

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        Stage1::from(input.iter().cloned().collect::<Vec<usize>>())
    }
}

impl<const N: usize> From<[usize; N]> for Stage1 {
    fn from(input: [usize; N]) -> Self {
        Stage1::from(Vec::from(input))
    }
}

impl From<Vec<usize>> for Stage1 {
    fn from(cups_vec: Vec<usize>) -> Self {
        Stage1 {
            cups: vec_to_hash_map(&cups_vec),
            start: *cups_vec.first().unwrap(),
        }
    }
}
