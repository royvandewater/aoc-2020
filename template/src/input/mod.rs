use std::str::FromStr;

pub struct Input;

impl FromStr for Input {
    type Err = String;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let _sut: Input = "".parse()?;
        Ok(())
    }
}
