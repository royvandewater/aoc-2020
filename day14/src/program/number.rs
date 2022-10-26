use super::instruction::Digit;
use super::instruction::Mask;
use super::util;

pub struct Number {
    pub value: u64,
}

impl Number {
    pub fn new() -> Number {
        Number { value: 0 }
    }

    pub fn apply(&mut self, mask: &Mask, value: u64) {
        let mut digits = util::to_digits(value);

        for (i, d) in mask.iter().enumerate() {
            match d {
                Digit::X => continue,
                _ => digits[i] = *d,
            }
        }

        self.value = util::from_digits(digits);
    }
}
