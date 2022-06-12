use super::Instruction;
use super::Operation;

impl TryFrom<&str> for Instruction {
  type Error = String;

  fn try_from(line: &str) -> Result<Self, Self::Error> {
    let mut parts = line.trim().split(" ");
    let operation_str = parts
      .next()
      .ok_or(format!("Instruction missing operation: {}", line))?;
    let operation = Operation::from_str(operation_str)?;

    let argument_str = parts
      .next()
      .ok_or(format!("Instruction missing argument: {}", line))?;

    let argument = match isize::from_str_radix(argument_str, 10) {
      Ok(i) => i,
      Err(_) => {
        return Err(format!("Failed to parse argument: {}", argument_str));
      }
    };

    Ok(Instruction {
      operation: operation,
      argument: argument,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_program() {
    assert!(Instruction::try_from("").is_err());
  }

  #[test]
  fn from_str_acc() -> Result<(), String> {
    let instruction = Instruction::try_from("acc +1")?;

    assert_eq!(Instruction::new(Operation::Acc, 0), instruction);
    Ok(())
  }

  #[test]
  fn from_str_jmp() -> Result<(), String> {
    let instruction = Instruction::try_from("jmp -2")?;

    assert_eq!(Instruction::new(Operation::Jmp, -2), instruction);
    Ok(())
  }

  #[test]
  fn from_str_nop() -> Result<(), String> {
    let instruction = Instruction::try_from("nop +0")?;

    assert_eq!(Instruction::new(Operation::Nop, 0), instruction);
    Ok(())
  }
}
