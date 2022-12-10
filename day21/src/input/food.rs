use std::{collections::HashSet, str::FromStr};

pub struct Food {
    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingredients_str, allergens_str) = s.split_once("(contains ").unwrap_or((s, ")"));

        let ingredients = ingredients_str
            .split(" ")
            .filter_map(|i| match i {
                "" => None,
                _ => Some(i.to_string()),
            })
            .collect();

        let allergens = allergens_str
            .replace(")", "")
            .split(", ")
            .filter_map(|i| match i {
                "" => None,
                _ => Some(i.to_string()),
            })
            .collect();

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let sut: Food = "".parse().unwrap();

        assert_eq!(HashSet::<String>::new(), sut.ingredients);
        assert_eq!(HashSet::<String>::new(), sut.allergens);
    }

    #[test]
    fn test_single_ingredient() {
        let sut: Food = "mxmxvkd".parse().unwrap();

        assert_eq!(HashSet::from(["mxmxvkd".to_string()]), sut.ingredients);
    }

    #[test]
    fn test_multiple_ingredients() {
        let sut: Food = "a b c".parse().unwrap();

        assert_eq!(
            HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()]),
            sut.ingredients
        );
    }

    #[test]
    fn test_single_allergen() {
        let sut: Food = "(contains soy)".parse().unwrap();

        assert_eq!(HashSet::from(["soy".to_string()]), sut.allergens);
    }

    #[test]
    fn test_multiple_of_both() {
        let sut: Food = "a b c (contains soy, fish)".parse().unwrap();

        assert_eq!(
            HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()]),
            sut.ingredients
        );
        assert_eq!(
            HashSet::from(["soy".to_string(), "fish".to_string()]),
            sut.allergens
        );
    }
}
