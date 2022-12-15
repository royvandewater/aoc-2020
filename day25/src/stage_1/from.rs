use crate::input::Input;

use super::Stage1;

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        Stage1 {
            card: input.card,
            door: input.door,
        }
    }
}

impl From<[usize; 2]> for Stage1 {
    fn from([card, door]: [usize; 2]) -> Self {
        Stage1 { card, door }
    }
}
