use std::collections::{HashMap, HashSet};

use crate::stage_1::Stage1;

mod from;

pub struct Stage2 {
    stage1: Stage1,
}

type AllergenMap = HashMap<String, String>;

impl Stage2 {
    pub fn answer(&self) -> String {
        let allergen_map = self.reduce(self.stage1.allergen_map());

        let mut sorted_allergens: Vec<String> = allergen_map.keys().cloned().collect();
        sorted_allergens.sort();

        let sorted_ingredients: Vec<String> = sorted_allergens
            .iter()
            .map(|allergen| allergen_map.get(allergen).unwrap().to_string())
            .collect();

        return sorted_ingredients.join(",");
    }

    fn reduce(&self, allergen_map: HashMap<String, HashSet<String>>) -> AllergenMap {
        let known_allergen_map: AllergenMap = allergen_map
            .iter()
            .filter_map(|(a, ingredients)| match ingredients.len() {
                1 => Some((a.to_string(), ingredients.iter().next().unwrap().to_string())),
                _ => None,
            })
            .collect();

        if known_allergen_map.len() == allergen_map.len() {
            return known_allergen_map;
        }

        let known_ingredients: HashSet<String> = known_allergen_map.values().cloned().collect();
        let mut reduced_map: HashMap<String, HashSet<String>> = HashMap::new();

        for (allergen, ingredients) in allergen_map.iter() {
            if known_allergen_map.contains_key(allergen) {
                reduced_map.insert(allergen.to_string(), ingredients.clone());
                continue;
            }

            reduced_map.insert(
                allergen.to_string(),
                ingredients.difference(&known_ingredients).cloned().collect(),
            );
        }

        return self.reduce(reduced_map);
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Input;

    use super::*;

    #[test]
    fn test_example_1() {
        let input: Input = "
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "
        .parse()
        .unwrap();

        let sut = Stage2::from(&input);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", sut.answer());
    }
}
