use super::instruction::Digit;
use super::instruction::Mask;

pub struct Number {
    pub value: u64,
}

impl Number {
    pub fn new() -> Number {
        Number { value: 0 }
    }

    pub fn apply(&mut self, mask: &Mask, value: u64) {
        let mut digits = to_digits(value);

        for (i, d) in mask.iter().enumerate() {
            match d {
                Digit::X => continue,
                _ => digits[i] = *d,
            }
        }

        let mut output_str = String::new();
        for digit in digits {
            output_str.push(match digit {
                Digit::Zero => '0',
                Digit::One => '1',
                Digit::X => panic!("There should be no X at this point"),
            })
        }

        self.value = u64::from_str_radix(&output_str, 2).unwrap();
    }
}

fn to_digits(value: u64) -> [Digit; 36] {
    let mut value = value;
    let mut digits_vec: Vec<Digit> = Vec::new();

    for i in 0..36 {
        let exp = (2 as u64).pow(35 - i);

        digits_vec.push(Digit::try_from(value / exp).unwrap());
        value = value % exp;
    }

    let digits: [Digit; 36] = digits_vec
        .try_into()
        .map_err(|v: Vec<Digit>| {
            format!(
                "Expected vector to contain exactly 36 elements, contained: {}",
                v.len()
            )
        })
        .unwrap();

    digits
}
