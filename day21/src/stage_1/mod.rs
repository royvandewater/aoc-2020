mod from;

pub struct Stage1;

impl Stage1 {
    pub fn answer(&self) -> Result<isize, String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let _sut: Stage1 = Stage1 {};
        Ok(())
    }
}
