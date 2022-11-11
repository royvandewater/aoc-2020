mod item;

use std::{slice::Iter, str::FromStr};

pub use self::item::Item;

pub struct Input(Vec<Item>);

impl Input {
    pub(crate) fn iter(&self) -> Iter<Item> {
        self.0.iter()
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_items = s.trim().split("\n\n");
        let items = raw_items
            .map(|raw_item| raw_item.parse())
            .collect::<Result<Vec<Item>, String>>()?;

        Ok(Input(items))
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
