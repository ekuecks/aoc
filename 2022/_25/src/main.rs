use std::io::stdin;

fn convert(line: String) -> i64 {
    let mut multiplier: i64 = 1;
    let mut num = 0;
    for c in line.chars().rev() {
        match c {
            '0' => {}
            '1' => num += multiplier,
            '2' => num += multiplier * 2,
            '-' => num -= multiplier,
            '=' => num -= multiplier * 2,
            _ => panic!(),
        }
        multiplier *= 5;
    }
    num
}

fn main() {
    let mut ans = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut multiplier: i64 = 1;
        let mut num = 0;
        for c in line.chars().rev() {
            match c {
                '0' => {}
                '1' => num += multiplier,
                '2' => num += multiplier * 2,
                '-' => num -= multiplier,
                '=' => num -= multiplier * 2,
                _ => panic!(),
            }
            multiplier *= 5;
        }
        ans += num;
    }
    let mut real_ans = "".to_string();
    let mut multiplier = 1;
    dbg!(ans);
    while 2 * multiplier < ans {
        multiplier *= 5;
    }
    while multiplier > 0 {
        dbg!(ans);
        dbg!(multiplier);
        let mut max_residual = 0;
        let mut resid_m = multiplier / 5;
        while resid_m > 0 {
            max_residual += 2 * resid_m;
            resid_m /= 5;
        }
        if ans >= multiplier * 3 {
            dbg!(real_ans);
            panic!();
        }
        if ans >= multiplier {
            if ans >= multiplier * 2 {
                real_ans.push('2');
                ans -= 2 * multiplier;
            } else {
                if ans - multiplier > max_residual { 
                    real_ans.push('2');
                    ans -= 2 * multiplier;
                } else {
                    real_ans.push('1');
                    ans -= multiplier;
                }
            }
        } else if ans > max_residual {
            real_ans.push('1');
            ans -= multiplier;
        } else if ans < 0 {
            if ans >= -multiplier {
                if ans + multiplier > max_residual {
                    real_ans.push('0');
                } else {
                    real_ans.push('-');
                    ans += multiplier;
                }
            } else {
                if ans + 2 * multiplier > max_residual {
                    real_ans.push('-');
                    ans += multiplier;
                } else {
                    real_ans.push('=');
                    ans += 2 * multiplier;
                }
            }
        } else {
            real_ans.push('0');
        }
        multiplier /= 5;
    }
    dbg!(&real_ans);
    dbg!(convert(real_ans));
    dbg!(ans);
}
