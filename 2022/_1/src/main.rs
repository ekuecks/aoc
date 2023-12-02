use std::io::stdin;

fn main() {
    let input = stdin();
    let mut cur: usize = 0;
    let mut m1= 0;
    let mut m2 = 0;
    let mut m3 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            cur = 0;
            continue;
        }
        cur += trimmed.parse::<usize>().unwrap();
        if cur >= m1 {
            m3 = m2;
            m2 = m1;
            m1 = cur;
        } else if cur >= m2 {
            m3 = m2;
            m2 = cur;
        } else if cur >= m3 {
            m3 = cur;
        }
    }
    dbg!(m1 + m2 + m3);
}
