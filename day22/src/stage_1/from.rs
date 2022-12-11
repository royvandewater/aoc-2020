use crate::input::Input;

use super::Stage1;

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        let player_1 = input.player_1.clone();
        let player_2 = input.player_2.clone();

        Stage1 { player_1, player_2 }
    }
}

impl From<[Vec<usize>; 2]> for Stage1 {
    fn from([player_1, player_2]: [Vec<usize>; 2]) -> Self {
        Stage1 { player_1, player_2 }
    }
}
