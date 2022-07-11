mod from_str;

pub struct Schedule {
    start_time: usize,
    intervals: Vec<usize>,
}

impl Schedule {
    pub fn answer(&self) -> usize {
        let mut remainders = self
            .intervals
            .iter()
            .map(|i| (i - (self.start_time % i), *i))
            .collect::<Vec<(usize, usize)>>();
        remainders.sort();
        let (wait_time, interval) = remainders.first().expect("Remainders was empty");

        println!("wait_time: {}, interval: {}", wait_time, interval);

        wait_time * interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_time_10_intervals_3() {
        let sut = Schedule {
            start_time: 10,
            intervals: vec![3],
        };

        assert_eq!(6, sut.answer())
    }

    #[test]
    fn example_1() {
        let sut = Schedule {
            start_time: 939,
            intervals: vec![7, 13, 59, 31, 19],
        };

        assert_eq!(295, sut.answer())
    }
}
