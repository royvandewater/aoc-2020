use std::collections::HashSet;

use crate::input::Food;

mod allergen_map;
mod from;

pub struct Stage1 {
    foods: Vec<Food>,
}

impl Stage1 {
    pub fn answer(&self) -> usize {
        let all_ingredients: HashSet<String> = self
            .foods
            .iter()
            .flat_map(|f| f.ingredients.clone())
            .collect();

        let ingredients_with_allergens: HashSet<String> = self
            .allergen_map()
            .iter()
            .flat_map(|(_, ingredients)| ingredients.clone())
            .collect();

        let ingredients_without_allergens = all_ingredients.difference(&ingredients_with_allergens);

        ingredients_without_allergens
            .map(|ingredient| {
                self.foods
                    .iter()
                    .filter(|f| f.ingredients.contains(ingredient))
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Input;

    use super::*;

    #[test]
    fn test_empty() {
        let sut: Stage1 = Stage1 { foods: Vec::new() };
        assert_eq!(0, sut.answer());
    }

    #[test]
    fn test_answer_example_1() {
        let input: Input = "
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "
        .parse()
        .unwrap();

        let sut = Stage1::from(&input);
        assert_eq!(5, sut.answer());
    }

    #[test]
    fn test_answer_simple() {
        let input: Input = "
            mxmxvkd
        "
        .parse()
        .unwrap();

        let sut = Stage1::from(&input);
        assert_eq!(1, sut.answer());
    }

    #[test]
    fn test_answer_one_ingredient_one_allergen() {
        let input: Input = "
            mxmxvkd (contains dairy)
        "
        .parse()
        .unwrap();

        let sut = Stage1::from(&input);
        assert_eq!(0, sut.answer());
    }

    #[test]
    fn test_ingredient_without_allergens_example_1() {}
}
