use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
struct S {
    cards: Vec<char>,
    pairs: usize,
    threes: usize,
    fours: usize,
    fives: usize,
    bid: usize,
}

impl FromStr for S {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().collect();
        let bid = bid.parse().unwrap();
        let mut counts: HashMap<char, usize> = HashMap::new();
        for &card in &cards {
            *counts.entry(card).or_default() += 1;
        }
        // Remove me for part1
        let jokers = counts.remove(&'J').unwrap_or(0);
        // Remove me for part1
        if jokers == 5 {
            return Ok(S {
                pairs: 0,
                threes: 0,
                fours: 0,
                fives: 1,
                bid,
                cards,
            });
        }
        let mut pairs = 0;
        let mut threes = 0;
        let mut fours = 0;
        let mut fives = 0;
        for count in counts.into_values() {
            if count == 2 {
                pairs += 1;
            } else if count == 3 {
                threes += 1;
            } else if count == 4 {
                fours += 1;
            } else if count == 5 {
                fives += 1;
            }
        }
        // Remove me for part1
        for _ in 0..jokers {
            if fours > 0 {
                fours -= 1;
                fives += 1;
                continue;
            }
            if threes > 0 {
                threes -= 1;
                fours += 1;
                continue;
            }
            if pairs > 0 {
                pairs -= 1;
                threes += 1;
                continue;
            }
            pairs += 1;
        }

        Ok(S {
            bid,
            pairs,
            threes,
            fours,
            fives,
            cards,
        })
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for S {
    fn cmp(&self, other: &S) -> Ordering {
        // Swap the ranking arrays for part1
        // let ranking = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
        let ranking = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
        let ordering = self
            .fives
            .cmp(&other.fives)
            .then(self.fours.cmp(&other.fours))
            .then(self.threes.cmp(&other.threes))
            .then(self.pairs.cmp(&other.pairs));
        if ordering == Ordering::Equal {
            for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
                if a != b {
                    let mut i = 14;
                    for &r in &ranking {
                        if r == a {
                            break;
                        }
                        i -= 1;
                    }
                    let mut j = 14;
                    for &r in &ranking {
                        if r == b {
                            break;
                        }
                        j -= 1;
                    }
                    return i.cmp(&j);
                }
            }
            return Ordering::Equal;
        }
        ordering
    }
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut inputs: Vec<_> = lines
        .into_iter()
        .map(|l| S::from_str(l.trim()).unwrap())
        .collect();
    inputs.sort();
    let mut sum = 0;
    for (i, hand) in inputs.into_iter().enumerate() {
        sum += (i + 1) * hand.bid;
    }
    dbg!(sum);
}
