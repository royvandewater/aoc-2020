use std::collections::HashMap;

mod from_str;

pub struct NumberStream(Vec<usize>);

impl NumberStream {
    pub fn nth(&self, n: usize) -> usize {
        let mut seq = self.0.clone();
        let mut last_number = seq.pop().unwrap();

        let mut known_numbers: HashMap<usize, usize> = seq
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, num)| (num, i + 1))
            .collect();

        for i in self.0.len()..n {
            let number = calculate_next_number(&known_numbers, last_number, i);
            known_numbers.insert(last_number, i);

            last_number = number;
        }

        return last_number;
    }
}

fn calculate_next_number(
    known_numbers: &HashMap<usize, usize>,
    last_number: usize,
    index: usize,
) -> usize {
    match known_numbers.get(&last_number) {
        None => 0,
        Some(i) => index - i,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_example_1_n_4() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(4));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_5() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(3, sut.nth(5));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_6() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(3, sut.nth(6));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_7() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(1, sut.nth(7));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_8() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(8));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_9() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(4, sut.nth(9));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_10() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(10));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(436, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_2_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 3, 2]);

        assert_eq!(1, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_3_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 1, 3]);

        assert_eq!(10, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_4_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 2, 3]);

        assert_eq!(27, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_5_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 3, 1]);

        assert_eq!(78, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_6_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 2, 1]);

        assert_eq!(438, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_7_n_2020() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 1, 2]);

        assert_eq!(1836, sut.nth(2020));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(175594, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_2_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 3, 2]);

        assert_eq!(2578, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_3_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 1, 3]);

        assert_eq!(3544142, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_5_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 2, 3]);

        assert_eq!(261214, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_5_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 3, 1]);

        assert_eq!(6895259, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_6_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 2, 1]);

        assert_eq!(18, sut.nth(30000000));
        Ok(())
    }

    #[test]
    fn test_nth_example_7_n_30000000() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 1, 2]);

        assert_eq!(362, sut.nth(30000000));
        Ok(())
    }
}
