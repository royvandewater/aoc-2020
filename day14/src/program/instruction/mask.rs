use std::{ops::Index, slice::Iter, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Digit {
    X,
    Zero,
    One,
}

impl TryFrom<char> for Digit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Digit::X),
            '0' => Ok(Digit::Zero),
            '1' => Ok(Digit::One),
            c => Err(format!("Unrecognized char: {}", c)),
        }
    }
}

impl TryFrom<u64> for Digit {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Digit::Zero),
            1 => Ok(Digit::One),
            c => Err(format!("Unrecognized value: {}", c)),
        }
    }
}

pub struct Mask([Digit; 36]);

impl Mask {
    pub fn new() -> Mask {
        let digits: [Digit; 36] = [Digit::X; 36];
        Mask(digits)
    }

    pub fn iter(&self) -> Iter<Digit> {
        self.0.iter()
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits_vec: Vec<Digit> = s
            .trim()
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Digit>, String>>()?;

        let digits: [Digit; 36] = digits_vec.try_into().map_err(|v: Vec<Digit>| {
            format!("expected digits to be 36 chars, was: {}.", v.len(),)
        })?;

        Ok(Mask(digits))
    }
}

impl Index<usize> for Mask {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_x_mask() -> Result<(), String> {
        let sut: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse()?;

        for item in sut.0.iter() {
            assert_eq!(Digit::X, *item)
        }

        Ok(())
    }

    #[test]
    fn valid_0_mask() -> Result<(), String> {
        let sut: Mask = "000000000000000000000000000000000000".parse()?;

        for item in sut.0.iter() {
            assert_eq!(Digit::Zero, *item)
        }

        Ok(())
    }

    #[test]
    fn valid_1_mask() -> Result<(), String> {
        let sut: Mask = "111111111111111111111111111111111111".parse()?;

        for item in sut.0.iter() {
            assert_eq!(Digit::One, *item)
        }

        Ok(())
    }

    #[test]
    fn invalid_mask() {
        let sut: Result<Mask, String> = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".parse();

        assert!(sut.is_err())
    }
}
