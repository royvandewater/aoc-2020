mod try_from;

use std::collections::HashSet;

pub use crate::instruction::Instruction;
pub use crate::instruction::Operation;

pub struct Program {
  instructions: Vec<Instruction>,

  accumulator: isize,
  position: usize,
  visited_positions: HashSet<usize>,
}

pub struct InfiniteLoopError;

impl Program {
  pub fn new(instructions: Vec<Instruction>) -> Program {
    Program {
      instructions: instructions,
      accumulator: 0,
      position: 0,
      visited_positions: HashSet::new(),
    }
  }

  pub fn run(self: &mut Self) -> Result<(), InfiniteLoopError> {
    loop {
      if self.visited_positions.contains(&self.position) {
        return Err(InfiniteLoopError);
      }
      if self.position == self.instructions.len() {
        return Ok(());
      }

      self.visited_positions.insert(self.position);

      let instruction = &self.instructions[self.position];
      let argument = instruction.argument;

      match instruction.operation {
        Operation::Acc => self.acc(argument),
        Operation::Jmp => self.jmp(argument),
        Operation::Nop => self.nop(),
      }
    }
  }

  pub fn run_until_loop_detected(self: &mut Self) {
    self.run().unwrap_or(())
  }

  pub fn get_accumulator(self: &Self) -> isize {
    self.accumulator
  }

  fn acc(self: &mut Self, argument: isize) {
    self.position += 1;
    self.accumulator += argument;
  }

  fn jmp(self: &mut Self, argument: isize) {
    let position: isize = self.position.try_into().unwrap_or(0);
    let new_position: isize = position + argument;

    self.position = new_position.try_into().unwrap_or(0);
  }

  fn nop(self: &mut Self) {
    self.position += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_until_loop_detected_minimal_program() -> Result<(), String> {
    let mut program = Program::try_from("jmp +0")?;
    program.run_until_loop_detected();
    assert_eq!(0, program.get_accumulator());
    Ok(())
  }

  #[test]
  fn run_until_loop_detected_alterred_acc() -> Result<(), String> {
    let mut program = Program::try_from(
      "
        acc +1
        jmp -1
      ",
    )?;
    program.run_until_loop_detected();
    assert_eq!(1, program.get_accumulator());
    Ok(())
  }

  #[test]
  fn run_with_loop() -> Result<(), String> {
    let mut program = Program::try_from("jmp +0")?;
    assert!(program.run().is_err());
    Ok(())
  }
}
