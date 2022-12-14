use std::str::FromStr;

use super::AdapterCounter;

impl FromStr for AdapterCounter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adapters: Vec<usize> = s
            .trim()
            .lines()
            .map(|line| -> Result<usize, String> {
                line.trim()
                    .parse()
                    .or(Err(format!("Error parsing adapter as usize: {}", line)))
            })
            .collect::<Result<Vec<usize>, String>>()?;

        if adapters.len() == 0 {
            return Err("Length of adapters was 0".into());
        }
        adapters.sort();

        Ok(AdapterCounter { adapters })
    }
}
