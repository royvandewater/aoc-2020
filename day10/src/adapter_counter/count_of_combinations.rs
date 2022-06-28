use std::collections::VecDeque;

use super::AdapterCounter;

impl AdapterCounter {
    pub fn count_of_combinations(&self) -> usize {
        let goal = self.adapters.iter().max().unwrap() + 3;
        let adapters: VecDeque<usize> = self.adapters.clone().into();

        count_of_combinations(goal, 0, &adapters)
    }
}

fn count_of_combinations(goal: usize, acc: usize, adapters: &VecDeque<usize>) -> usize {
    if adapters.len() == 0 {
        if goal - acc <= 3 {
            return 1;
        }
        return 0;
    }

    let mut sum = 0;
    let mut rest = adapters.clone();

    loop {
        match rest.pop_front() {
            None => return sum,
            Some(adapter) => {
                if adapter - acc <= 3 {
                    sum += count_of_combinations(goal, adapter, &rest);
                    continue;
                }
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_valid_adapter() {
        let sut = AdapterCounter { adapters: vec![1] };
        assert_eq!(1, sut.count_of_combinations());
    }

    #[test]
    fn example_1() -> Result<(), String> {
        let sut: AdapterCounter = "
            16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4
        "
        .parse()?;

        assert_eq!(8, sut.count_of_combinations());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), String> {
        let sut: AdapterCounter = "
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "
        .parse()?;

        assert_eq!(19208, sut.count_of_combinations());
        Ok(())
    }
}
