use std::collections::VecDeque;

mod from;
mod round;

pub struct Stage1(VecDeque<usize>);

impl Stage1 {
    pub fn answer(&self) -> String {
        let mut cards = self.0.clone();

        for _ in 0..100 {
            cards = round::round(cards);
        }

        return cards_to_string(cards);
    }
}

fn cards_to_string(mut cards: VecDeque<usize>) -> String {
    while cards.front().unwrap() != &1 {
        cards.rotate_left(1);
    }

    cards
        .iter()
        .skip(1)
        .map(|d| format!("{}", d))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_example_1() {
        let sut: Stage1 = Stage1(VecDeque::from([3, 8, 9, 1, 2, 5, 4, 6, 7]));

        assert_eq!("67384529", sut.answer())
    }

    #[test]
    fn test_cards_to_string() {
        assert_eq!(
            "92658374",
            cards_to_string(VecDeque::from([5, 8, 3, 7, 4, 1, 9, 2, 6]))
        )
    }
}
