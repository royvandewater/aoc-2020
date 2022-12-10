use crate::input::Input;

use super::Stage2;

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        Stage2 { stage1: input.into() }
    }
}
