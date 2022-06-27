mod from_str;

pub struct AdapterCounter {
    adapters: Vec<usize>,
}

impl AdapterCounter {
    pub fn get_counts(&self) -> (usize, usize) {
        let mut ones = 0;
        let mut threes = 0;

        let mut previous = 0;

        for adapter in &self.adapters {
            let diff = adapter - previous;

            match diff {
                1 => ones += 1,
                3 => threes += 1,
                _ => panic!(
                    "Received invalid diff: {} - {} = {}",
                    adapter, previous, diff
                ),
            }

            previous = *adapter;
        }

        threes += 1;

        (ones, threes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let (ones, threes) = sut.get_counts();

        assert_eq!(7, ones);
        assert_eq!(5, threes);
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

        let (ones, threes) = sut.get_counts();

        assert_eq!(22, ones);
        assert_eq!(10, threes);
        Ok(())
    }
}
