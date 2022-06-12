use super::Repairer;
use crate::instruction::Instruction;

impl TryFrom<&str> for Repairer {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Repairer {
      accumulator: 0,
      instructions: Instruction::parse_many(value)?,
    })
  }
}
