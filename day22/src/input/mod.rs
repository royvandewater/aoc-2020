use std::{num::ParseIntError, str::FromStr};

pub struct Input {
    pub player_1: Vec<usize>,
    pub player_2: Vec<usize>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (player_1_str, player_2_str) = s.trim().split_once("\n\n").ok_or("Failed to find two decks")?;

        Ok(Input {
            player_1: parse_player(player_1_str).map_err(|e| format!("Failed to parse player_1: {}", e))?,
            player_2: parse_player(player_2_str).map_err(|e| format!("Failed to parse player_2: {}", e))?,
        })
    }
}

fn parse_player(player_str: &str) -> Result<Vec<usize>, ParseIntError> {
    player_str.lines().skip(1).map(|l| usize::from_str(l.trim())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(Input::from_str("").is_err());
    }

    #[test]
    fn test_one_card_each() {
        let sut: Input = "
        Player 1:
        2

        Player 2:
        3
        "
        .parse()
        .unwrap();

        assert_eq!(vec![2], sut.player_1);
        assert_eq!(vec![3], sut.player_2);
    }
}
