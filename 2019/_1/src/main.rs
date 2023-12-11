use std::io::stdin;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let inputs: Vec<_> = lines.into_iter().map(|s| s.trim().parse::<isize>().unwrap()).collect();
    let mut part1 = 0;
    let mut part2 = 0;
    for input in inputs {
        let mut fuel = input / 3 - 2;
        part1 += fuel;
        part2 += fuel;
        while fuel / 3 - 2 > 0 {
            fuel = fuel / 3 - 2;
            part2 += fuel;
        }
    }
    dbg!(part1);
    dbg!(part2);
}
