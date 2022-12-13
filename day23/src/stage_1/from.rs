use crate::input::Input;

use super::Stage1;

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        Stage1(input.iter().cloned().collect())
    }
}
