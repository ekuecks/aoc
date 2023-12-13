use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn main() {
    let input = stdin();
    let mut possible_matches: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients_count = HashMap::new();
    loop {
        let mut buf = String::new();
        let bytes = input.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        let mut split = buf.trim().split("(");
        let ingredients: HashSet<_> = split
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.to_owned())
            .collect();
        for ingredient in &ingredients {
            ingredients_count
                .entry(ingredient.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let mut allergens = Vec::new();
        for mut allergen in split.next().unwrap().split(" ") {
            if allergen == "contains" {
                continue;
            }
            allergen = &allergen[0..allergen.len() - 1];
            println!("{}", allergen);
            if allergen.trim().is_empty() {
                continue;
            }
            allergens.push(allergen.to_owned());
        }
        for allergen in allergens {
            match possible_matches.get_mut(&allergen) {
                Some(possible) => {
                    for p in possible.clone() {
                        if !ingredients.contains(&p) {
                            possible.remove(&p);
                        }
                    }
                }
                None => {
                    possible_matches.insert(allergen, ingredients.clone());
                }
            }
        }
    }
    let mut all_possible = HashSet::new();
    for (_, possible) in &possible_matches {
        for p in possible {
            all_possible.insert(p.clone());
        }
    }
    let mut total = 0;
    let mut inert = HashSet::new();
    for (ingredient, count) in &ingredients_count {
        if !all_possible.contains(ingredient) {
            inert.insert(ingredient.clone());
            total += count;
        }
    }
    let mut done = false;
    let mut isolated = HashSet::new();
    while !done {
        done = true;
        for (key, possible) in possible_matches.clone() {
            if isolated.contains(&key) {
                continue;
            }
            if possible.len() == 1 {
                isolated.insert(key.clone());
                let ingredient = possible.iter().next().unwrap().clone();
                done = false;
                for (other_key, other_possible) in possible_matches.iter_mut() {
                    if other_key == &key {
                        continue;
                    }
                    other_possible.remove(&ingredient);
                }
            }
        }
    }

    let mut sorted_keys = possible_matches
        .iter()
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();
    sorted_keys.sort();
    let mut answer = String::new();
    for key in sorted_keys {
        answer.push_str(
            possible_matches
                .get(&key)
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .as_str(),
        );
        answer.push_str(",");
    }

    println!("{:?}", possible_matches);
    println!("{}", answer);

    println!("{}", total);
}
