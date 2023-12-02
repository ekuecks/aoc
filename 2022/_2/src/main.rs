use std::io::stdin;

fn calc_score(opp: char, me: char) -> usize {
    let opp = opp as usize - 'A' as usize;
    let me = me as usize - 'X' as usize;
    if me == 0 {
        (opp + 2) % 3 + 1
    } else if me == 1 {
        opp + 4
    } else {
        (opp + 1) % 3 + 7
    }
}

fn main() {
    let mut score = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        let opp = chars[0];
        let me = chars[2];
        score += calc_score(opp, me);
    }
    dbg!(score);
}
