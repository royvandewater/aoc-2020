mod operation;
mod try_from;

pub use operation::Operation;

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
  pub operation: Operation,
  pub argument: isize,
}

impl Instruction {
  pub fn new(operation: Operation, argument: isize) -> Instruction {
    Instruction {
      operation: operation,
      argument: argument,
    }
  }

  pub fn parse_many(input: &str) -> Result<Vec<Instruction>, String> {
    let lines = input.trim().lines();
    lines.map(|l| Instruction::try_from(l)).collect()
  }
}
