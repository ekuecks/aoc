use std::io::stdin;

#[derive(Debug, PartialEq, Eq, Default, Hash)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Cost {
    fn from_str(s: &str) -> Self {
        let mut cost = Cost::default();
        let mut parts = s.split(" costs ");
        let _ = parts.next();
        let desc = parts.next().unwrap();
        let parts = desc.trim_end_matches(|c| c == '.').split(" and ");
        for part in parts {
            let mat_parts: Vec<_> = part.split(' ').collect();
            assert_eq!(mat_parts.len(), 2);
            let quantity = mat_parts[0].parse::<usize>().unwrap();
            if mat_parts[1] == "ore" {
                cost.ore = quantity;
            }
            if mat_parts[1] == "clay" {
                cost.clay = quantity;
            }
            if mat_parts[1] == "obsidian" {
                cost.obsidian = quantity;
            }
        }
        cost
    }

    fn can_build(&self, inventory: &Inventory) -> bool {
        inventory.ore >= self.ore
            && inventory.clay >= self.clay
            && inventory.obsidian >= self.obsidian
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
struct Blueprint {
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
struct RobotCollection {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Inventory {
    fn build(&mut self, cost: &Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }

    fn update(&mut self, collection: &RobotCollection) {
        self.ore += collection.ore;
        self.clay += collection.clay;
        self.obsidian += collection.obsidian;
        self.geode += collection.geode;
    }
}

fn optimize(
    left: usize,
    inventory: Inventory,
    collection: RobotCollection,
    blueprint: &Blueprint,
    so_far: usize,
    mut opt: usize,
) -> usize {
    if left <= 1 {
        return opt;
    }
    if opt > so_far + (left * (left - 1)) / 2 {
        return opt;
    }
    let mut new_inventory = inventory.clone();
    new_inventory.update(&collection);
    // build geode
    if blueprint.geode_cost.can_build(&inventory) {
        let mut inventory = new_inventory.clone();
        inventory.build(&blueprint.geode_cost);
        let mut collection = collection.clone();
        collection.geode += 1;
        let so_far = so_far + left - 1;
        opt = opt.max(so_far);
        opt = opt.max(optimize(
            left - 1,
            inventory,
            collection,
            blueprint,
            so_far,
            opt,
        ))
    }
    // build ore
    if blueprint.ore_cost.can_build(&inventory) {
        let mut inventory = new_inventory.clone();
        inventory.build(&blueprint.ore_cost);
        let mut collection = collection.clone();
        collection.ore += 1;
        opt = opt.max(optimize(
            left - 1,
            inventory,
            collection,
            blueprint,
            so_far,
            opt,
        ))
    }
    // build clay
    if blueprint.clay_cost.can_build(&inventory) {
        let mut inventory = new_inventory.clone();
        inventory.build(&blueprint.clay_cost);
        let mut collection = collection.clone();
        collection.clay += 1;
        opt = opt.max(optimize(
            left - 1,
            inventory,
            collection,
            blueprint,
            so_far,
            opt,
        ))
    }
    // build obsidian
    if blueprint.obsidian_cost.can_build(&inventory) {
        let mut inventory = new_inventory.clone();
        inventory.build(&blueprint.obsidian_cost);
        let mut collection = collection.clone();
        collection.obsidian += 1;
        opt = opt.max(optimize(
            left - 1,
            inventory,
            collection,
            blueprint,
            so_far,
            opt,
        ))
    }
    // don't build
    opt = opt.max(optimize(
        left - 1,
        new_inventory,
        collection,
        blueprint,
        so_far,
        opt,
    ));
    opt
}

fn main() {
    // robot order is ore, clay, obsidian, geode
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        let mut parts = l.split(':');
        parts.next().unwrap();
        let cost_description = &parts.next().unwrap()[1..];
        let descriptions: Vec<_> = cost_description.split(". ").collect();
        let blueprint = Blueprint {
            ore_cost: Cost::from_str(descriptions[0]),
            clay_cost: Cost::from_str(descriptions[1]),
            obsidian_cost: Cost::from_str(descriptions[2]),
            geode_cost: Cost::from_str(descriptions[3]),
        };
        blueprints.push(blueprint);
    }
    let mut ans = 1;
    for (i, blueprint) in blueprints.into_iter().enumerate() {
        if i > 2 {
            continue;
        }
        dbg!(i);
        let inventory = Inventory::default();
        let mut collection = RobotCollection::default();
        collection.ore += 1;
        let max = optimize(32, inventory, collection, &blueprint, 0, 0);
        dbg!(max);
        ans *= max;
    }
    dbg!(ans);
}
