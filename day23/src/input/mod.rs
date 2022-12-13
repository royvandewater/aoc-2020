use std::str::FromStr;

pub struct Input(Vec<usize>);

impl Input {
    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.0.iter()
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        str.chars()
            .map(|c| usize::from_str(c.to_string().as_str()))
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|e| format!("Failed to parse input: {}", e))
            .map(Input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let sut: Input = "".parse().unwrap();

        assert_eq!(Vec::<&usize>::new(), sut.iter().collect::<Vec<&usize>>())
    }

    #[test]
    fn test_1_value() {
        let sut: Input = "1".parse().unwrap();

        assert_eq!(vec![&1], sut.iter().collect::<Vec<_>>())
    }
}
