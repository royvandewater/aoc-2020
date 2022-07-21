use std::str::FromStr;

use super::{instruction::Instruction, Program};

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .trim()
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Instruction>, String>>()?;

        Ok(Program {
            instructions,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: Program = "".parse()?;

        assert_eq!(0, sut.instructions.len());
        Ok(())
    }

    #[test]
    fn test_mask() -> Result<(), String> {
        let sut: Program = "
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        "
        .parse()?;

        assert_eq!(1, sut.instructions.len());
        Ok(())
    }

    #[test]
    fn test_mask_and_mem() -> Result<(), String> {
        let sut: Program = "
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[1] = 2
        "
        .parse()?;

        assert_eq!(2, sut.instructions.len());
        Ok(())
    }
}
