mod mask;
mod mem;

pub use mask::Digit;
pub use mask::Mask;
use mem::Mem;
use std::str::FromStr;

pub enum Instruction {
    Mask(Mask),
    Mem(Mem),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split("=");
        let key = parts.next().ok_or("could not find key")?.trim();

        match key {
            _ if key == "mask" => {
                let mask: Mask = parts.next().ok_or("Could not find value")?.trim().parse()?;
                Ok(Instruction::Mask(mask))
            }
            _ if key.starts_with("mem") => {
                let mem: Mem = s.parse()?;
                Ok(Instruction::Mem(mem))
            }
            _ => Err(format!("Unknown instruction type: {}", key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() -> Result<(), String> {
        let sut: Instruction = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse()?;

        match sut {
            Instruction::Mask(_) => Ok(()),
            _ => Err("Instruction was not a mask".into()),
        }
    }

    #[test]
    fn test_mem() -> Result<(), String> {
        let sut: Instruction = "mem[8] = 11".parse()?;

        match sut {
            Instruction::Mem(m) => {
                assert_eq!(8, m.address);
                assert_eq!(11, m.value);
                Ok(())
            }
            _ => Err("Instruction was not a Mem value".into()),
        }
    }
}
