use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let inputs: Vec<_> = lines
        .clone()
        .into_iter()
        .map(|s| {
            let (l, r) = s.split_once(' ').unwrap();
            let l = l.chars().next().unwrap();
            let (r, _) = r.split_once(' ').unwrap();
            let r: isize = r.parse().unwrap();
            (l, r)
        })
        .collect();
    let inputs2: Vec<_> = lines
        .clone()
        .into_iter()
        .map(|s| {
            let (_, r) = s.split_once('#').unwrap();
            let s = &r[0..(r.len() - 1)];
            let len = isize::from_str_radix(&s[0..s.len() - 1], 16).unwrap();
            let dir = match &s[(s.len() - 1)..] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => unreachable!(),
            };
            (dir, len)
        })
        .collect();
    for inputs in [inputs, inputs2] {
        let mut pos = (0, 0);
        let mut points = Vec::new();
        let mut perimeter = 0;
        for (d, len) in inputs {
            let dir = match d {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => unreachable!(),
            };
            pos.0 += len * dir.0;
            pos.1 += len * dir.1;
            points.push(pos);
            perimeter += len;
        }
        assert_eq!(points[points.len() - 1], (0, 0));
        let mut total: isize = 0;
        for i in 0..points.len() {
            let j = if i == points.len() - 1 { 0 } else { i + 1 };
            total += points[i].0 * points[j].1 - points[i].1 * points[j].0;
        }
        let ans = ((total.abs() + perimeter) / 2) + 1;
        dbg!(ans);
    }
}
