use std::io::stdin;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

struct Card {
    id: usize,
    numbers: HashSet<usize>,
    winners: HashSet<usize>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.split_once(":").unwrap();
        let id = first.trim().rsplit_once(" ").unwrap().1.parse().unwrap();
        let (first, rest) = rest.split_once(" | ").unwrap();
        let winners = first.trim().split(" ").flat_map(|s| s.parse::<usize>().ok()).collect();
        let numbers = rest.trim().split(" ").flat_map(|s| s.parse::<usize>().ok()).collect();
        Ok(Card { id, numbers, winners })
    }
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut part1 = 0;
    let mut copies = VecDeque::new();
    let mut part2 = 0;
    for line in lines.iter() {
        let card_copies = copies.pop_front().unwrap_or(0) + 1;
        part2 += card_copies;
        let line = line.trim();
        let card = Card::from_str(line).unwrap();
        let matches = card.numbers.intersection(&card.winners).collect::<Vec<_>>().len() as u32;
        for i in 0..matches {
            let i = i as usize;
            if copies.len() == i {
                copies.push_back(card_copies);
            } else {
                copies[i] += card_copies;
            }
        }
        if matches > 0 {
            part1 += 2_usize.pow(matches - 1);
        }
    }
    dbg!(part1);
    dbg!(part2);
}