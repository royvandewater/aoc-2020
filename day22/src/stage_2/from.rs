use crate::input::Input;

use super::Stage2;

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        let player_1 = input.player_1.clone();
        let player_2 = input.player_2.clone();

        Stage2 { player_1, player_2 }
    }
}

impl From<[Vec<usize>; 2]> for Stage2 {
    fn from([player_1, player_2]: [Vec<usize>; 2]) -> Self {
        Stage2 { player_1, player_2 }
    }
}
