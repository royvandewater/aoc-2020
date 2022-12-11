use std::collections::VecDeque;

mod from;

pub struct Stage1 {
    player_1: Vec<usize>,
    player_2: Vec<usize>,
}

impl Stage1 {
    pub fn answer(&self) -> usize {
        let winner = play(self.player_1.clone(), self.player_2.clone());
        winner.iter().rev().enumerate().map(|(i, card)| (i + 1) * card).sum()
    }
}

fn play(player_1: Vec<usize>, player_2: Vec<usize>) -> Vec<usize> {
    let (player_1, player_2) = round(player_1, player_2);

    if player_1.is_empty() {
        return player_2.clone();
    }

    if player_2.is_empty() {
        return player_1.clone();
    }

    return play(player_1, player_2);
}

fn round(player_1: Vec<usize>, player_2: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut player_1: VecDeque<usize> = player_1.into();
    let mut player_2: VecDeque<usize> = player_2.into();

    let card_1 = player_1.pop_front().unwrap();
    let card_2 = player_2.pop_front().unwrap();

    if card_1 > card_2 {
        player_1.push_back(card_1);
        player_1.push_back(card_2);
    }

    if card_1 < card_2 {
        player_2.push_back(card_2);
        player_2.push_back(card_1);
    }

    return (player_1.into(), player_2.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_one_card_each() {
        let sut: Stage1 = [vec![1], vec![2]].into();
        assert_eq!(5, sut.answer())
    }

    #[test]
    fn test_answer_example_1() {
        let sut: Stage1 = [vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]].into();
        assert_eq!(306, sut.answer())
    }

    #[test]
    fn test_round_example_1() {
        let (player_1, player_2) = round(vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]);

        assert_eq!(vec![2, 6, 3, 1, 9, 5], player_1);
        assert_eq!(vec![8, 4, 7, 10], player_2);
    }
}
