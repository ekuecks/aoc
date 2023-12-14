use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut data = Vec::new();
    for line in lines {
        let v: Vec<_> = line.chars().collect();
        data.push(v);
    }
    // NWSE
    let mut cache = HashMap::new();
    let mut use_cache = true;
    let mut count: u64 = 0;
    let l = data.len();
    let m = data[0].len();
    let target: u64 = 1000000000;
    while count < target {
        // N
        count += 1;
        for i in 0..l {
            for j in 0..m {
                let c = data[i][j];
                if c == 'O' {
                    let mut found = false;
                    for k in 1..=i {
                        let d = data[i - k][j];
                        if d == '#' || d == 'O' {
                            data[i - k + 1][j] = 'O';
                            if k != 1 {
                                data[i][j] = '.';
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        data[0][j] = 'O';
                        if i != 0 {
                            data[i][j] = '.';
                        }
                    }
                }
            }
        }

        if count == 1 {
            let mut part1 = 0;
            for (i, row) in data.iter().enumerate() {
                for elem in row {
                    if *elem == 'O' {
                        part1 += data.len() - i;
                    }
                }
            }
            dbg!(part1);
        }
        // W
        for row in data.iter_mut() {
            for j in 0..m {
                let c = row[j];
                if c == 'O' {
                    let mut found = false;
                    for k in 1..=j {
                        let d = row[j - k];
                        if d == '#' || d == 'O' {
                            row[j - k + 1] = 'O';
                            if k != 1 {
                                row[j] = '.';
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        row[0] = 'O';
                        if j != 0 {
                            row[j] = '.';
                        }
                    }
                }
            }
        }
        // S
        for i in (0..l).rev() {
            for j in 0..m {
                let c = data[i][j];
                if c == 'O' {
                    let mut found = false;
                    for k in i + 1..l {
                        let d = data[k][j];
                        if d == '#' || d == 'O' {
                            data[k - 1][j] = 'O';
                            if k != i + 1 {
                                data[i][j] = '.';
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        data[l - 1][j] = 'O';
                        if i != l - 1 {
                            data[i][j] = '.';
                        }
                    }
                }
            }
        }
        // E
        for row in data.iter_mut() {
            for j in (0..m).rev() {
                let c = row[j];
                if c == 'O' {
                    let mut found = false;
                    for k in j + 1..m {
                        let d = row[k];
                        if d == '#' || d == 'O' {
                            row[k - 1] = 'O';
                            if k != j + 1 {
                                row[j] = '.';
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        row[m - 1] = 'O';
                        if j != m - 1 {
                            row[j] = '.';
                        }
                    }
                }
            }
        }
        if use_cache {
            if let Some(prev) = cache.get(&data) {
                let diff = count - *prev;
                let steps = (target - count) / diff;
                count += diff * steps;
                use_cache = false;
            }
            cache.insert(data.clone(), count);
        }
    }

    let mut part2 = 0;
    for (i, row) in data.iter().enumerate() {
        for elem in row {
            if *elem == 'O' {
                part2 += data.len() - i;
            }
        }
    }
    dbg!(part2);
}
