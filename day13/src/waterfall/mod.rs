mod from_str;
mod interval;

pub struct Waterfall {
    intervals: Vec<usize>,
}

impl Waterfall {
    pub fn start_time(&self) -> usize {
        let mut current = 0;
        let mut step_size = 1;

        for (offset, interval) in self.intervals.iter().enumerate() {
            for t in (current..usize::MAX).step_by(step_size) {
                if (t + offset) % interval == 0 {
                    current = t;
                    step_size *= interval;
                    break;
                }
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::Waterfall;

    #[test]
    fn example_1() {
        let sut = Waterfall {
            intervals: vec![7, 13, 1, 1, 59, 1, 31, 19],
        };

        assert_eq!(1068781, sut.start_time())
    }

    struct Case {
        expected: usize,
        intervals: Vec<usize>,
    }

    fn parse_case(s: &str) -> Case {
        let mut pieces = s.split("|");

        let expected: usize = pieces.next().unwrap().trim().parse().unwrap();
        let intervals: Vec<usize> = pieces
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|i| i.parse::<usize>().unwrap_or(1))
            .collect();

        Case {
            expected,
            intervals,
        }
    }

    #[test]
    fn table() {
        let cases = "
            1  | 1,x,3
            0  | 1,x,2
            2  | 1,x,4
            2  | 2,x,4
            4  | 2,x,3
            28 | 7,x,5
        "
        .trim()
        .lines()
        .map(parse_case);

        for case in cases {
            let sut = Waterfall {
                intervals: case.intervals.clone(),
            };

            assert_eq!(
                case.expected,
                sut.start_time(),
                "intervals: {:?}",
                case.intervals
            );
        }
    }
}
