use crate::{
    input::{Direction, Input},
    stage_1::Stage1,
};

use super::Stage2;

impl From<Input> for Stage2 {
    fn from(input: Input) -> Self {
        Stage2::from(&input)
    }
}

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        let stage1 = Stage1::from(input);

        Stage2(stage1.black_tiles())
    }
}

impl<const N: usize, const M: usize> From<[[Direction; M]; N]> for Stage2 {
    fn from(input: [[Direction; M]; N]) -> Self {
        Stage2::from(&Input::from(input))
    }
}

impl<const N: usize> From<[Vec<Direction>; N]> for Stage2 {
    fn from(input: [Vec<Direction>; N]) -> Self {
        Stage2::from(&Input::from(input))
    }
}
