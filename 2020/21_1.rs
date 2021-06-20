use std::collections::{HashMap, HashSet};
use std::io::Read;

// Observations
// 1. All lines ends with ')', so no need to check for that

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

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let parsed: Vec<_> = buffer.trim().split('\n').map(parse).collect();

    let mut allergen_map: HashMap<String, HashSet<String>> = HashMap::new();
    for (ingredients, allergens) in parsed.iter() {
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

    let found: HashSet<String> = allergen_map.into_iter().map(|x| x.1).flatten().collect();
    let count: usize = parsed
        .into_iter()
        .map(|x| x.0)
        .flatten()
        .filter(|x| !found.contains(&x[..]))
        .count();

    println!("{}", count);
}
