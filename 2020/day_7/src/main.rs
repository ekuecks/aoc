use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;
use std::sync::{Arc, Mutex};

mod part_1 {
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct Bag {
        pub id: String,
        pub can_be_contained_in: Vec<Arc<Mutex<Bag>>>,
    }

    impl Bag {
        pub fn new(id: String) -> Self {
            Self {
                id,
                can_be_contained_in: Vec::new(),
            }
        }
    }
}

mod part_2 {
    use std::cmp::{Eq, PartialEq};
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct Bag {
        pub id: String,
        pub contains: HashMap<String, usize>,
    }

    impl Bag {
        pub fn new(id: String) -> Self {
            Self {
                id,
                contains: HashMap::new(),
            }
        }
    }

    impl Hash for Bag {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl PartialEq for Bag {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for Bag {}
}

fn main() {
    let input = stdin();
    let mut map = HashMap::new();
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let mut parts = line.split(" contain ");
                let container_id = parts.next().unwrap();
                let container_id = container_id.get(0..container_id.len() - 1).unwrap(); // Remove trailing 's'
                let container = map
                    .entry(container_id.to_owned())
                    .or_insert_with(|| {
                        Arc::new(Mutex::new(part_2::Bag::new(container_id.to_owned())))
                    })
                    .clone();
                println!("Container: {}", container_id);
                let rest = parts.next().unwrap();
                let rest = rest.trim();
                let rest = rest.get(0..rest.len() - 1).unwrap(); // Remove ending '.'
                for contained_str in rest.split(", ") {
                    if contained_str == "no other bags" {
                        continue;
                    }
                    println!("Contained str: {}", contained_str);
                    let contained_str_trimmed = if &contained_str[0..1] != "1" {
                        contained_str.get(0..contained_str.len() - 1).unwrap() // Remove trailing 's'
                    } else {
                        contained_str
                    };
                    let number: usize = contained_str_trimmed[0..1].parse().unwrap();
                    let contained_id = contained_str_trimmed.splitn(2, " ").nth(1).unwrap(); // Remove number and space
                    assert!(!contained_id.contains("bags"), contained_str.to_owned());
                    container
                        .lock()
                        .unwrap()
                        .contains
                        .insert(contained_id.to_owned(), number);
                }
            }
            e => panic!("Failed to read input: {:?}", e),
        }
    }
    println!("{}", get_contained_bags("shiny gold bag".to_owned(), &map));
}

fn get_contained_bags(next: String, map: &HashMap<String, Arc<Mutex<part_2::Bag>>>) -> usize {
    let bag = map.get(&next).unwrap();
    println!("{:?}", bag);
    let mut total = 0;
    for (id, count) in bag.lock().unwrap().contains.iter() {
        total += count * (1 + get_contained_bags(id.clone(), map));
    }
    total
}
