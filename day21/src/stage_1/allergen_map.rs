use std::collections::{HashMap, HashSet};

use crate::input::Food;

use super::Stage1;

impl Stage1 {
    pub(crate) fn allergen_map(&self) -> HashMap<String, HashSet<String>> {
        let ingredients: HashSet<String> = self
            .foods
            .iter()
            .flat_map(|food| food.ingredients.iter().cloned())
            .collect();
        let allergens: HashSet<String> = self
            .foods
            .iter()
            .flat_map(|food| food.allergens.iter().cloned())
            .collect();

        // let allergen_map = HashMap::<String, HashSet<String>>::new();
        let mut allergen_map: HashMap<String, HashSet<String>> = allergens
            .iter()
            .map(|allergen| (allergen.to_string(), ingredients.iter().cloned().collect()))
            .collect();

        for allergen in allergens.iter() {
            let foods: Vec<&Food> = self
                .foods
                .iter()
                .filter(|food| food.allergens.contains(allergen))
                .collect();

            let mut ingredients = allergen_map.get(allergen).unwrap().clone();
            for food in foods {
                ingredients = ingredients
                    .intersection(&food.ingredients)
                    .map(|i| i.to_string())
                    .collect();
            }
            allergen_map.insert(allergen.to_string(), ingredients);
        }

        return allergen_map;
    }
}

#[cfg(test)]
mod tests {
    // use crate::input::Input;

    use super::*;

    #[test]
    fn test_empty() {
        let sut: Stage1 = Stage1 { foods: Vec::new() };

        assert_eq!(HashMap::new(), sut.allergen_map());
    }

    #[test]
    fn test_one_ingredient() {
        let sut: Stage1 = Stage1 {
            foods: vec!["foo".parse().unwrap()],
        };

        assert_eq!(HashMap::new(), sut.allergen_map());
    }

    #[test]
    fn test_one_ingredient_one_allergen() {
        let sut: Stage1 = Stage1 {
            foods: vec!["sushi (contains fish)".parse().unwrap()],
        };

        assert_eq!(
            own(HashMap::from([("fish", HashSet::from(["sushi"]))])),
            sut.allergen_map()
        );
    }

    fn own(input: HashMap<&str, HashSet<&str>>) -> HashMap<String, HashSet<String>> {
        input
            .iter()
            .map(|(key, value)| (key.to_string(), own_hash_set(value)))
            .collect()
    }

    fn own_hash_set(input: &HashSet<&str>) -> HashSet<String> {
        input.iter().map(|v| v.to_string()).collect()
    }
}
