use super::Instruction;
use super::Program;

impl TryFrom<&str> for Program {
  type Error = String;

  fn try_from(input: &str) -> Result<Self, Self::Error> {
    let instructions = Instruction::parse_many(input)?;

    Ok(Program::new(instructions))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::program::*;

  #[test]
  fn from_str_empty_program() -> Result<(), String> {
    let program = Program::try_from("")?;
    assert_eq!(0, program.instructions.len());
    Ok(())
  }

  #[test]
  fn from_str_one_of_each() -> Result<(), String> {
    let p = Program::try_from(
      "
        acc -1
        jmp +1
        nop +0
      ",
    )?;

    assert_eq!(3, p.instructions.len());
    assert_eq!(Instruction::new(Operation::Acc, -1), p.instructions[0]);
    assert_eq!(Instruction::new(Operation::Jmp, 1), p.instructions[1]);
    assert_eq!(Instruction::new(Operation::Nop, 0), p.instructions[2]);
    Ok(())
  }
}
