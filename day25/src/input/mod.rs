use std::str::FromStr;

pub struct Input {
    pub card: usize,
    pub door: usize,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();

        let first_line = lines
            .next()
            .ok_or("Could not retrieve first line of input".to_string())?
            .trim();

        let second_line = lines
            .next()
            .ok_or("Could not retrieve second line of input".to_string())?
            .trim();

        Ok(Input {
            card: usize::from_str(first_line)
                .map_err(|e| format!("Failed to parse first line as usize: {}", e))?,
            door: usize::from_str(second_line)
                .map_err(|e| format!("Failed to parse second line as usize: {}", e))?,
        })
    }
}

impl From<[usize; 2]> for Input {
    fn from([first, second]: [usize; 2]) -> Self {
        Input {
            card: first,
            door: second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_numbers() {
        let sut: Input = "
            1
            2
        "
        .parse()
        .unwrap();

        assert_eq!(1, sut.card);
        assert_eq!(2, sut.door);
    }
}
