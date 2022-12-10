mod from;

pub struct Stage2;

impl Stage2 {
    pub fn answer(&self) -> Result<isize, String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let _sut: Stage2 = Stage2 {};
        Ok(())
    }
}
