use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use regex::Regex;

use crate::inputs::read_contents;

struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Debug for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IngredientList(\n\tIngredients: {:?}\n\tAllergens: {:?}\n)", self.ingredients, self.allergens)
    }
}

fn parse_input(input: &str) -> Vec<Recipe> {
    let re = Regex::new(r"^(.+) \(contains ([^)]+)\)$").unwrap();

    input.lines()
         .map(|line| {
             let caps = re.captures(line.trim()).unwrap();

             let ingredients = caps.get(1)
                                   .unwrap()
                                   .as_str()
                                   .split_ascii_whitespace()
                                   .map(|s| s.trim().to_string())
                                   .collect();

             let allergens = caps.get(2)
                                 .unwrap()
                                 .as_str()
                                 .split(", ")
                                 .map(|s| s.trim().to_string())
                                 .collect();

             Recipe { ingredients, allergens }
         })
         .collect()
}

pub fn solve_a() {
    let recipes = parse_input(&read_contents(21));
    let list = check_allergens(&recipes);
    let ans = count_appearance(&list, &recipes);

    println!("Solution A: {}", ans);
}

fn check_allergens(recipes: &Vec<Recipe>) -> HashMap<String, HashSet<String>> {
    let mut ingredient_allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut allergen_checklist: HashMap<String, HashSet<String>> = HashMap::new();

    for recipe in recipes {
        for ingredient in recipe.ingredients.iter() {
            ingredient_allergens.entry(ingredient.clone())
                                .and_modify(|allergens| allergens.extend(recipe.allergens.clone()))
                                .or_insert(recipe.allergens.clone());
        }
    }

    for recipe in recipes {
        for allergen in recipe.allergens.iter() {
            if allergen_checklist.contains_key(allergen) {
                let ingredients = allergen_checklist.get_mut(allergen).unwrap();
                let full = ingredients.union(&recipe.ingredients).cloned().collect::<HashSet<_>>();

                ingredients.retain(|e| recipe.ingredients.contains(e));

                full.difference(ingredients)
                    .for_each(|e| {
                        let allergens = ingredient_allergens.get_mut(e).unwrap();
                        allergens.remove(allergen);
                    });
            } else {
                allergen_checklist.insert(allergen.clone(), recipe.ingredients.clone());
            }
        }
    }

    let mut checked = HashSet::new();
    while let Some((allergen, ingredients)) = allergen_checklist.iter()
                                                                .find(|&(a, ig)| ig.len() == 1 && !checked.contains(a))
                                                                .map(|v| (v.0.clone(), v.1.clone())) {
        let ingredient = ingredients.iter().next().unwrap();
        checked.insert(allergen.clone());

        ingredient_allergens.insert(ingredient.clone(), HashSet::from([allergen.clone()]));

        allergen_checklist.iter_mut().for_each(|(other_allergen, other_ingredients)| {
            if *other_allergen != allergen {
                other_ingredients.remove(ingredient);
            }
        });
    }


    ingredient_allergens
}

fn count_appearance(ingredient_allergens: &HashMap<String, HashSet<String>>, recipes: &Vec<Recipe>) -> usize {
    let allergen_free = ingredient_allergens.iter()
                                            .filter(|(_, v)| v.is_empty()).map(|(k, _)| k.clone())
                                            .collect::<HashSet<_>>();

    recipes.iter()
           .map(|recipe| recipe.ingredients.intersection(&allergen_free).count())
           .sum()
}

pub fn solve_b() {
    let recipes = parse_input(&read_contents(21));
    let list = check_allergens(&recipes);

    let ans = isolate_allergens(&list);
    println!("Solution B: {}", ans);
}

fn isolate_allergens(ingredient_allergens: &HashMap<String, HashSet<String>>) -> String {
    ingredient_allergens.iter()
                        .filter(|(_, v)| !v.is_empty())
                        .map(|(k, v)| {
                            assert_eq!(v.len(), 1, "'{}' has more than one possible allergen", k);

                            (v.iter().next().unwrap().clone(), k)
                        })
                        .sorted_by(|(allergen, _), (other, _)| allergen.cmp(other))
                        .map(|(_, ingredient)| ingredient)
                        .join(",")
}

#[cfg(test)]
mod tests {
    use super::{check_allergens, count_appearance, isolate_allergens, parse_input};

    fn test_input<'a>() -> &'a str {
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
    }

    #[test]
    fn test_count_appearance() {
        let recipes = parse_input(test_input());
        println!("{:?}", recipes);

        let list = check_allergens(&recipes);
        let ans = count_appearance(&list, &recipes);

        assert_eq!(ans, 5);
    }

    #[test]
    fn test_isolate_allergens() {
        let recipes = parse_input(test_input());
        let list = check_allergens(&recipes);

        let ans = isolate_allergens(&list);
        assert_eq!(ans, "mxmxvkd,sqjhc,fvjkl");
    }
}