use crate::input::{Direction, Input, Instruction};

use super::Stage1;

impl From<Input> for Stage1 {
    fn from(input: Input) -> Self {
        Stage1::from(&input)
    }
}

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        let instructions: Vec<Instruction> = input.iter().cloned().collect();

        Stage1(instructions)
    }
}

impl<const N: usize, const M: usize> From<[[Direction; M]; N]> for Stage1 {
    fn from(input: [[Direction; M]; N]) -> Self {
        Stage1::from(&Input::from(input))
    }
}

impl<const N: usize> From<[Vec<Direction>; N]> for Stage1 {
    fn from(input: [Vec<Direction>; N]) -> Self {
        Stage1::from(&Input::from(input))
    }
}
