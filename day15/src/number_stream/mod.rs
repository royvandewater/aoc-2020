mod from_str;

pub struct NumberStream(Vec<usize>);

impl NumberStream {
    pub fn nth(&self, n: usize) -> usize {
        let mut seq = self.0.clone();

        for i in 0..=n {
            if i < seq.len() {
                continue;
            }

            let number = calculate_next_number(&seq);
            seq.push(number)
        }

        return *seq.last().unwrap();
    }
}

fn calculate_next_number(seq: &Vec<usize>) -> usize {
    let last_number = seq.last().unwrap();

    for (i, number) in seq.iter().enumerate().rev().skip(1) {
        if number == last_number {
            return seq.len() - (i + 1);
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_example_1_n_3() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(3));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_4() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(3, sut.nth(4));
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

        assert_eq!(1, sut.nth(6));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_7() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(7));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_8() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(4, sut.nth(8));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_9() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(0, sut.nth(9));
        Ok(())
    }

    #[test]
    fn test_nth_example_1_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![0, 3, 6]);

        assert_eq!(436, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_2_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 3, 2]);

        assert_eq!(1, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_3_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 1, 3]);

        assert_eq!(10, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_4_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![1, 2, 3]);

        assert_eq!(27, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_5_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![2, 3, 1]);

        assert_eq!(78, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_6_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 2, 1]);

        assert_eq!(438, sut.nth(2019));
        Ok(())
    }

    #[test]
    fn test_nth_example_7_n_2019() -> Result<(), String> {
        let sut: NumberStream = NumberStream(vec![3, 1, 2]);

        assert_eq!(1836, sut.nth(2019));
        Ok(())
    }
}
