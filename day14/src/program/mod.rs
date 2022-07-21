mod from_str;
mod instruction;
mod number;

use std::collections::HashMap;

use instruction::Instruction;
use number::Number;

use self::instruction::Mask;

pub struct Program {
    instructions: Vec<Instruction>,
    memory: HashMap<u64, Number>,
}

impl Program {
    pub fn run(&mut self) {
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

    pub fn sequence_sum(&self) -> u64 {
        self.memory.values().map(|n| n.value).sum()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            memory: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<(), String> {
        let mut sut: Program = "
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "
        .parse()?;

        sut.run();
        assert_eq!(165, sut.sequence_sum());

        Ok(())
    }
}
