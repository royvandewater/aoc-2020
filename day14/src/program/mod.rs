mod from_str;
mod instruction;
mod number;
mod util;

use std::collections::HashMap;

use instruction::Instruction;
use number::Number;

use self::{
    instruction::{Digit, Mask},
    util::to_digits,
};

pub struct Program {
    instructions: Vec<Instruction>,
    memory: HashMap<u64, Number>,
    memory2: HashMap<u64, u64>,
}

impl Program {
    pub fn run(&mut self) {
        self.stage_1();
        self.stage_2();
    }

    fn stage_1(&mut self) {
        let mut mask: &Mask = &Mask::new();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask(m) => mask = m,
                Instruction::Mem(m) => {
                    let address = m.address;
                    if !self.memory.contains_key(&address) {
                        self.memory.insert(address, Number::new());
                    }

                    self.memory.get_mut(&address).unwrap().apply(&mask, m.value);
                }
            }
        }
    }

    fn stage_2(&mut self) {
        let mut mask: &Mask = &Mask::new();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask(m) => mask = m,
                Instruction::Mem(m) => {
                    for address in compute_addresses(*mask, m.address) {
                        self.memory2.insert(address, m.value);
                    }
                }
            }
        }
    }

    pub fn stage1_sum(&self) -> u64 {
        self.memory.values().map(|n| n.value).sum()
    }

    pub fn stage2_sum(&self) -> u64 {
        self.memory2.values().sum()
    }
}

fn compute_addresses(mask: Mask, address: u64) -> Vec<u64> {
    let mut digits: [Digit; 36] = to_digits(address);

    for (i, digit) in mask.iter().enumerate() {
        match digit {
            Digit::X => digits[i] = Digit::X,
            Digit::Zero => continue,
            Digit::One => digits[i] = Digit::One,
        }
    }

    expand_addresses(digits)
}

fn expand_addresses(digits: [Digit; 36]) -> Vec<u64> {
    if !digits.contains(&Digit::X) {}

    for (i, digit) in digits.iter().enumerate() {
        match digit {
            Digit::X => {
                let mut digits_0 = digits;
                digits_0[i] = Digit::Zero;
                let res_0 = expand_addresses(digits_0);

                let mut digits_1 = digits;
                digits_1[i] = Digit::One;
                let res_1 = expand_addresses(digits_1);

                return [res_0, res_1].concat();
            }
            _ => continue,
        }
    }

    return vec![util::from_digits(digits)];
}

impl Default for Program {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            memory: HashMap::new(),
            memory2: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_1_example_1() -> Result<(), String> {
        let mut sut: Program = "
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "
        .parse()?;

        sut.run();
        assert_eq!(165, sut.stage1_sum());

        Ok(())
    }

    #[test]
    fn stage_2_example_2() -> Result<(), String> {
        let mut sut: Program = "
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        "
        .parse()?;

        sut.run();
        assert_eq!(208, sut.stage2_sum());

        Ok(())
    }
}
