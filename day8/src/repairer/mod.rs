mod try_from;

use crate::instruction::Instruction;
use crate::instruction::Operation;
use crate::program::Program;

pub struct Repairer {
  accumulator: isize,
  instructions: Vec<Instruction>,
}

impl Repairer {
  pub fn repair_and_run(self: &mut Self) -> Result<(), String> {
    for (i, instruction) in self.instructions.clone().iter().enumerate() {
      let mut instructions = self.instructions.clone();
      instructions[i] = repair_instruction(instruction);
      let mut program = Program::new(instructions);

      match program.run() {
        Err(_) => continue,
        Ok(_) => {
          self.accumulator = program.get_accumulator();
          return Ok(());
        }
      };
    }

    return Err("No permutations found where the program finishes".into());
  }

  pub fn get_accumulator(self: &Self) -> isize {
    self.accumulator
  }
}

fn repair_instruction(instruction: &Instruction) -> Instruction {
  match instruction.operation {
    Operation::Acc => instruction.clone(),
    Operation::Jmp => Instruction::new(Operation::Nop, instruction.argument),
    Operation::Nop => Instruction::new(Operation::Jmp, instruction.argument),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_instructions() -> Result<(), String> {
    let mut repairer = Repairer::try_from("")?;
    assert!(repairer.repair_and_run().is_err());
    Ok(())
  }

  #[test]
  fn example_1() -> Result<(), String> {
    let mut repairer = Repairer::try_from(
      "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
      ",
    )?;

    repairer.repair_and_run()?;
    assert_eq!(8, repairer.get_accumulator());
    Ok(())
  }
}
