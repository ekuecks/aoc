use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

#[derive(Clone)]
enum ModuleType {
    Broadcast(Vec<String>),
    Ff(bool, Vec<String>),
    Conj {
        inputs: HashMap<String, bool>,
        outputs: Vec<String>,
    },
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut modules = HashMap::new();
    let mut conjs = HashSet::new();
    for line in lines {
        let line = line.trim();
        let (l, r) = line.split_once(" -> ").unwrap();
        let ms = r.split(", ").map(|s| s.to_string()).collect();
        let name = l[1..].to_string();
        match &l[0..1] {
            "%" => {
                modules.insert(name, ModuleType::Ff(false, ms));
            }
            "&" => {
                modules.insert(
                    name.clone(),
                    ModuleType::Conj {
                        inputs: HashMap::new(),
                        outputs: ms,
                    },
                );
                conjs.insert(name);
            }
            _ => {
                assert_eq!(l, "broadcaster");
                modules.insert(l.to_string(), ModuleType::Broadcast(ms));
            }
        }
    }
    let c = modules.clone();
    for (name, module) in c {
        let outputs = match module {
            ModuleType::Ff(_, outputs) => outputs,
            ModuleType::Conj { outputs, .. } => outputs,
            ModuleType::Broadcast(outputs) => outputs,
        };
        for output in outputs {
            if let Some(ModuleType::Conj { inputs, .. }) = modules.get_mut(&output) {
                inputs.insert(name.clone(), false);
            }
        }
    }
    let mut lows = 0;
    let mut highs = 0;
    let mut cycle_lengths = HashMap::new();
    let Some(ModuleType::Conj { inputs, .. }) = modules.get("xm") else {
        unreachable!()
    };
    let notable_inputs: HashSet<String> = inputs.keys().cloned().collect();
    let mut i: usize = 0;
    loop {
        if i == 1000 {
            let part1 = highs * lows;
            dbg!(part1);
        }
        i += 1;
        let mut signals = VecDeque::new();
        signals.push_back((None, "broadcaster".to_string(), false));
        while let Some((from, name, signal)) = signals.pop_front() {
            if signal {
                highs += 1;
            } else {
                lows += 1;
            }
            let module = match modules.get_mut(&name) {
                Some(m) => m,
                None => {
                    if !signal {
                        let part2 = i;
                        dbg!(part2);
                        return;
                    }
                    continue;
                }
            };
            match module {
                ModuleType::Ff(state, outputs) => {
                    if !signal {
                        let current = *state;
                        *state = !current;
                        for output in outputs {
                            signals.push_back((Some(name.clone()), output.clone(), *state));
                        }
                    }
                }
                ModuleType::Conj { inputs, outputs } => {
                    let from = from.unwrap();
                    inputs.insert(from, signal);
                    if inputs.values().all(|b| *b) {
                        for output in outputs {
                            signals.push_back((Some(name.clone()), output.clone(), false));
                        }
                    } else {
                        for output in outputs {
                            signals.push_back((Some(name.clone()), output.clone(), true));
                        }
                        if notable_inputs.contains(&name) {
                            cycle_lengths.entry(name).or_insert(i);
                            if cycle_lengths.len() == notable_inputs.len() {
                                let mut part2 = 1;
                                for &value in cycle_lengths.values() {
                                    part2 = num::integer::lcm(part2, value);
                                }
                                dbg!(part2);
                                return;
                            }
                        }
                    }
                }
                ModuleType::Broadcast(outputs) => {
                    for output in outputs {
                        signals.push_back((Some(name.clone()), output.clone(), signal));
                    }
                }
            }
        }
    }
}
