use bacon_sci::interp::lagrange;
use std::io::stdin;

fn main() {
    let inputs: Vec<Vec<f64>> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let mut part1 = 0.;
    let mut part2 = 0.;
    for input in inputs {
        // Evaluate at 0 to avoid precision issues. Really evaluating f(-(x-N)) (from the .rev())
        let poly1 = lagrange(
            &(1..=(input.len() as isize))
                .map(|i| i as f64)
                .rev()
                .collect::<Vec<_>>(),
            &input,
            1e-6,
        )
        .unwrap();
        part1 += poly1.evaluate(0f64);
        let poly2 = lagrange(
            &(1..=(input.len() as isize))
                .map(|i| i as f64)
                .collect::<Vec<_>>(),
            &input,
            1e-6,
        )
        .unwrap();
        part2 += poly2.evaluate(0f64);
    }
    dbg!(part1);
    dbg!(part2);
}
