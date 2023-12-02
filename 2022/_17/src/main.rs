use std::io::stdin;
use std::collections::HashSet;


fn collides(rock: &Vec<(i32, i32)>, pos: (i32, i32), occupied: &HashSet<(i32, i32)>) -> bool {
    let (x, y) = pos;
    for &(rx, ry) in rock {
        let pos = (x + rx, y + ry);
        if occupied.contains(&pos) {
            return true;
        }
        if pos.0 >= 7 {
            return true;
        }
        if pos.0 < 0 {
            return true;
        }
    }
    false
}


fn main() {
    let line = stdin().lines().next().unwrap().unwrap();
    let l = line.trim();
    let shifts: Vec<i32> = l.chars().map(|c| if c == '<' { -1 } else { 1 }).collect();
    let mut occupied = HashSet::new();
    for x in 0..7 {
        occupied.insert((x, 0));
    }
    let mut height = 0;
    let mut rocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut rock_index = 0;
    let mut shift_index = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut has_seen = false;
    let mut last = 0;
    let mut last_i = 0;
    for num in 0..1_000_000_000_000_usize{
        let mut x = 2;
        let mut y = 4 + height;
        let rock = &rocks[rock_index];
        let key = (rock_index, shift_index);
        if num >= 5050 && (num - 5050) % 1725 == 0 {
            dbg!(height);
            dbg!(height - last);
            dbg!(num);
            dbg!(num - last_i);
            last_i = num;
            last = height;
        }
        // if !has_seen {
        //     if seen.contains(&key) {
        //         dbg!(&key);
        //         dbg!(height);
        //         has_seen = true;
        //     }
        //     seen.insert(key);
        // }
        loop {
            // shift
            let shift = shifts[shift_index];
            if (!collides(rock, (x + shift, y), &occupied)) {
                x += shift;
            }
            shift_index += 1;
            shift_index %= shifts.len();
            // down
            if (!collides(rock, (x, y - 1), &occupied)) {
                y -= 1;
            } else {
                for &(rx, ry) in rock {
                    occupied.insert((x + rx, y + ry));
                    height = height.max(y + ry);
                }
                break;
            }
        }
        rock_index += 1;
        rock_index %= rocks.len();
        if num == 10 {
            for y in (0..=height).rev() {
                for x in 0..7 {
                    if occupied.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            } 
        }
    }
    dbg!(height);
}
