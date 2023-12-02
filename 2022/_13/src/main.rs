use std::io::stdin;
use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Item {
    List(VecDeque<Item>),
    Int(u32),
}

impl Item {
    fn new_single_list(item: Self) -> Self {
        let mut v = VecDeque::new();
        v.push_back(item);
        Self::List(v)
    }

    fn compare(self, other: Item) -> Ordering {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => {
                a.cmp(&b)
            },
            (Item::List(mut a), Item::List(mut b)) => {
                loop {
                    match (a.pop_front(), b.pop_front()) {
                        (None, None) => return Ordering::Equal,
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(a), Some(b)) => {
                            match a.compare(b) {
                                Ordering::Equal => {},
                                ord => return ord,
                            }
                        }
                    }
                }
            },
            (Item::Int(a), Item::List(b)) => {
                Item::new_single_list(Item::Int(a)).compare(Item::List(b))
            },
            (Item::List(a), Item::Int(b)) => {
                Item::List(a).compare(Item::new_single_list(Item::Int(b)))
            },
        }
    }
}

#[derive(Debug)]
enum StackItem {
    Start,
    Item(Item),
}

fn main() {
    let mut packets = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let mut stack = VecDeque::new();
        let elems = l.split(',');
        for elem in elems {
            let chars: Vec<_> = elem.chars().collect();
            let mut i = 0;
            while chars[i] == '[' {
                stack.push_back(StackItem::Start);
                i += 1;
            }
            let mut x = String::new();
            while i < chars.len() && chars[i] != ']' {
                x.push(chars[i]);
                i += 1;
            }
            if !x.is_empty() {
                stack.push_back(StackItem::Item(Item::Int(x.parse::<u32>().unwrap())));
            }
            if i < chars.len() && chars[i] == ']' {
                while i < chars.len() {
                    let mut v = VecDeque::new();
                    while let Some(StackItem::Item(item)) = stack.pop_back() {
                        v.push_front(item);
                    }
                    stack.push_back(StackItem::Item(Item::List(v)));
                    i += 1;
                }
            }
        }
        assert!(stack.len() == 1);
        match stack.pop_back().unwrap() {
            StackItem::Item(Item::List(l)) => {
                packets.push(Item::List(l));
            }
            _ => panic!("Invalid stack"),
        }
    }
    let first = Item::new_single_list(Item::new_single_list(Item::Int(2)));
    packets.push(first.clone());
    let second = Item::new_single_list(Item::new_single_list(Item::Int(6)));
    packets.push(second.clone());
    packets.sort_unstable_by(|a, b| a.clone().compare(b.clone()));
    let l = packets.len();
    let mut first_i = l;
    let mut second_i = l;
    for (i, packet) in packets.into_iter().enumerate() {
        if first_i == l {
            if packet.compare(first.clone()) == Ordering::Equal {
                first_i = i + 1;
            }
        } else if packet.compare(second.clone()) == Ordering::Equal {
            second_i = i + 1;
            break;
        }
    }
    println!("{}", first_i * second_i);
}
