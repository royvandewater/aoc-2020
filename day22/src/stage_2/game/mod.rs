use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Debug, PartialEq)]
pub enum Winner {
    Player1,
    Player2,
}

#[derive(Eq, Hash, PartialEq)]
pub struct State {
    deck_1: VecDeque<usize>,
    deck_2: VecDeque<usize>,
}

pub fn game(deck_1: Vec<usize>, deck_2: Vec<usize>) -> (Winner, Vec<usize>) {
    let (winner, deck) = game_inner(deck_1.into(), deck_2.into(), &mut HashMap::new());

    (winner, deck.into())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn game_inner(
    mut deck_1: VecDeque<usize>,
    mut deck_2: VecDeque<usize>,
    mut known_games: &mut HashMap<u64, Winner>,
) -> (Winner, VecDeque<usize>) {
    let game_state = calculate_hash(&(&deck_1, &deck_2));

    if let Some(winner) = known_games.get(&game_state) {
        return match winner {
            Winner::Player1 => (Winner::Player1, deck_1),
            Winner::Player2 => (Winner::Player2, deck_2),
        };
    }

    let mut known_states: HashSet<u64> = HashSet::new();

    loop {
        let state = calculate_hash(&(&deck_1, &deck_2));
        if known_states.contains(&state) {
            known_games.insert(game_state, Winner::Player1);
            return (Winner::Player1, deck_1);
        }
        known_states.insert(state);

        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        let winner = match both_players_have_enough_cards(&card_1, &deck_1, &card_2, &deck_2) {
            true => {
                game_inner(
                    deck_1.iter().take(card_1).cloned().collect(),
                    deck_2.iter().take(card_2).cloned().collect(),
                    &mut known_games,
                )
                .0
            }
            false => single_round(&card_1, &card_2),
        };

        match winner {
            Winner::Player1 => {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            }
            Winner::Player2 => {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        }

        if deck_2.is_empty() {
            known_games.insert(game_state, Winner::Player1);
            return (Winner::Player1, deck_1.into());
        }

        if deck_1.is_empty() {
            known_games.insert(game_state, Winner::Player2);
            return (Winner::Player2, deck_2.into());
        }
    }
}

fn both_players_have_enough_cards(
    &card_1: &usize,
    deck_1: &VecDeque<usize>,
    &card_2: &usize,
    deck_2: &VecDeque<usize>,
) -> bool {
    deck_1.len() >= card_1 && deck_2.len() >= card_2
}

fn single_round(card_1: &usize, card_2: &usize) -> Winner {
    match card_1 > card_2 {
        true => Winner::Player1,
        false => Winner::Player2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_card_each() {
        let (winner, _) = game(vec![1], vec![2]);
        assert_eq!(Winner::Player2, winner);
    }

    #[test]
    fn test_example_1() {
        let (winner, _) = game(vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]);
        assert_eq!(Winner::Player2, winner);
    }

    #[test]
    fn test_recursive_loop() {
        let (winner, _) = game(vec![43, 19], vec![2, 29, 14]);
        assert_eq!(Winner::Player1, winner);
    }
}
