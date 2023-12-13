#![allow(dead_code)]
use std::io::stdin;

fn part_1() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let departure_time: u64 = buf.trim().parse().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let times: Vec<u64> = buf
        .trim()
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut min = departure_time;
    let mut min_time = 0;
    for time in times {
        let delta = time - departure_time % time;
        if delta < min {
            min = delta;
            min_time = time;
        }
    }
    println!("{}", min * min_time);
}

fn part_2() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let _departure_time: u64 = buf.trim().parse().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut times: Vec<(u64, u64)> = Vec::new();
    let mut index = 0;
    for s in buf.trim().split(",") {
        index += 1;
        if s == "x" {
            continue;
        }
        let time = s.parse().unwrap();
        let i = index % time;
        times.push(((time - i + 1) % time, time));
    }

    let mut total: u64 = 0;
    let mut prod: u64 = 1;
    for tup in times {
        let r = tup.0;
        let d = tup.1;
        while total % d != r {
            total += prod;
        }
        prod *= d;
    }
    println!("{}", total);
}

fn main() {
    part_2();
}
