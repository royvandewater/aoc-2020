use std::collections::{HashMap, HashSet};

use super::Stage1;

impl Stage1 {
    pub(crate) fn allergen_map(&self) -> HashMap<String, HashSet<String>> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    // use crate::input::Input;

    use super::*;

    #[test]
    fn test_empty() {
        let sut: Stage1 = Stage1 { food: Vec::new() };

        assert_eq!(HashMap::new(), sut.allergen_map());
    }

    #[test]
    fn test_one_ingredient() {
        let sut: Stage1 = Stage1 {
            food: vec!["foo".parse().unwrap()],
        };

        assert_eq!(HashMap::new(), sut.allergen_map());
    }

    #[test]
    fn test_one_ingredient_one_allergen() {
        let sut: Stage1 = Stage1 {
            food: vec!["sushi (contains fish)".parse().unwrap()],
        };

        assert_eq!(
            HashMap::from([(String::from("fish"), HashSet::from([String::from("sushi")]))]),
            sut.allergen_map()
        );
    }
}
