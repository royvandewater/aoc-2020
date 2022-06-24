mod try_from;
mod weakness;

pub use weakness::Weakness;

pub struct WeaknessFinder {
    sequence: Vec<usize>,
}

impl WeaknessFinder {
    pub fn find_weakness(self: &Self, input: usize) -> Result<Weakness, String> {
        for (i, x) in self.sequence.iter().enumerate() {
            let other = self.sequence.clone().split_off(i + 1);
            let mut acc: Vec<usize> = vec![*x];

            for y in other {
                acc.push(y);

                let sum: usize = acc.iter().sum();
                if input == sum {
                    return Ok(Weakness {
                        min: *acc.iter().min().ok_or("Could not find min value")?,
                        max: *acc.iter().max().ok_or("Could not find max value")?,
                    });
                }
                if input < sum {
                    break;
                }
            }
        }

        Err("No weakness found in sequence for input".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sequence() {
        let sut = WeaknessFinder { sequence: vec![] };
        let res = sut.find_weakness(0);
        assert!(res.is_err());
    }

    #[test]
    fn contains_weakness() -> Result<(), String> {
        let sut = WeaknessFinder {
            sequence: vec![1, 2],
        };
        let weakness = sut.find_weakness(3)?;

        assert_eq!(Weakness { min: 1, max: 2 }, weakness);
        Ok(())
    }

    #[test]
    fn example_1() -> Result<(), String> {
        let sut = WeaknessFinder {
            sequence: vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
        };

        let weakness = sut.find_weakness(127)?;

        assert_eq!(Weakness { min: 15, max: 47 }, weakness);

        Ok(())
    }
}
