#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
  Acc,
  Jmp,
  Nop,
}

impl Operation {
  pub fn from_str(input: &str) -> Result<Operation, String> {
    match input {
      "acc" => Ok(Operation::Acc),
      "jmp" => Ok(Operation::Jmp),
      "nop" => Ok(Operation::Nop),
      _ => Err(format!("Invalid operation: {}", input)),
    }
  }
}
