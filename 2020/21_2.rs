use std::collections::{HashMap, HashSet};
use std::io::Read;

// Observations
// 1. All lines ends with ')', so no need to check for that

type StrToHStr = HashMap<String, HashSet<String>>;

fn parse(line: &str) -> (Vec<String>, Vec<String>) {
    let v: Vec<_> = line.split("(contains").collect();
    assert!(v.len() == 2);

    let ingredients: Vec<_> = v[0]
        .split(' ')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    let allergens: Vec<_> = v[1][..v[1].len() - 1]
        .split(", ")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    (ingredients, allergens)
}

fn build_allergen_map(lines: &[(Vec<String>, Vec<String>)]) -> StrToHStr {
    let mut allergen_map: StrToHStr = HashMap::new();
    for (ingredients, allergens) in lines.iter() {
        for allergen in allergens {
            if !allergen_map.contains_key(allergen) {
                allergen_map.insert(allergen.to_string(), ingredients.iter().cloned().collect());
            } else {
                let n = allergen_map[&allergen[..]]
                    .intersection(&ingredients.iter().cloned().collect::<HashSet<_>>())
                    .cloned()
                    .collect();
                allergen_map.insert(allergen.to_string(), n);
            }
        }
    }
    allergen_map
}

fn build_ingredient_map(allergen_map: &StrToHStr) -> StrToHStr {
    let mut ingredient_map = HashMap::new();
    for (allergen, ingredients) in allergen_map {
        for ingredient in ingredients {
            let entry = ingredient_map
                .entry(ingredient.to_string())
                .or_insert_with(HashSet::new);
            entry.insert(allergen.to_string());
        }
    }
    ingredient_map
}

fn match_ingredients(allergen_map: &StrToHStr) -> Vec<(String, String)> {
    let mut single_allergens: Vec<String> = Vec::new();
    let mut allergen_map = allergen_map.clone();
    let mut ingredient_map = build_ingredient_map(&allergen_map);

    for (allergen, v) in allergen_map.iter() {
        assert!(!v.is_empty());
        if v.len() == 1 {
            single_allergens.push(allergen.to_string());
        }
    }

    let mut out = Vec::new();

    while !single_allergens.is_empty() {
        let allergen = single_allergens.pop().unwrap();
        let ingredient: String = allergen_map[&allergen[..]]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()[0]
            .to_string();

        for allergen in ingredient_map.remove(&ingredient[..]).unwrap() {
            assert!(allergen_map
                .get_mut(&allergen[..])
                .unwrap()
                .remove(&ingredient[..]));
            if allergen_map[&allergen[..]].len() == 1 {
                single_allergens.push(allergen);
            }
        }

        allergen_map.remove(&allergen[..]).unwrap();
        out.push((allergen, ingredient));
    }

    out.sort_unstable();
    out
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let parsed: Vec<_> = buffer.trim().split('\n').map(parse).collect();
    let allergen_map = build_allergen_map(&parsed);

    let matched_ingredients = match_ingredients(&allergen_map);
    println!(
        "{}",
        matched_ingredients
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<_>>()
            .join(",")
    );
}
