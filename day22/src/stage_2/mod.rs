mod from;
mod game;

pub struct Stage2 {
    player_1: Vec<usize>,
    player_2: Vec<usize>,
}

impl Stage2 {
    pub fn answer(&self) -> usize {
        let (_, deck) = game::game(self.player_1.clone(), self.player_2.clone());

        deck.iter().rev().enumerate().map(|(i, card)| (i + 1) * card).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_one_card_each() {
        let sut: Stage2 = [vec![1], vec![2]].into();
        assert_eq!(5, sut.answer())
    }

    #[test]
    fn test_answer_example_1() {
        let sut: Stage2 = [vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10]].into();
        assert_eq!(291, sut.answer())
    }
}
