use std::io::stdin;

struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

fn main() {
    let mut sensors = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.trim().split(' ').collect();
        let sx = parts[2][2..parts[2].len()-1].parse::<i64>().unwrap();
        let sy = parts[3][2..parts[3].len()-1].parse::<i64>().unwrap();
        let bx = parts[8][2..parts[8].len()-1].parse::<i64>().unwrap();
        let by = parts[9][2..].parse::<i64>().unwrap();
        let dist = (sx - bx).abs() + (sy - by).abs();
        sensors.push(Sensor {
            x: sx,
            y: sy,
            dist,
        });
    }
    for x in 0..=4_000_000 {
        if x % 100_000 == 0 {
            println!("{}", x);
        }
        let mut y = 0;
        while y <= 4_000_000 {
            let mut uncovered = true;
            for sensor in &sensors {
                let dx = (x - sensor.x).abs();
                let dy = (y - sensor.y).abs();
                let dist = dx + dy;
                if dist <= sensor.dist {
                    uncovered = false;
                    y = sensor.dist - dx + sensor.y;
                    break;
                }
            }
            if uncovered {
                println!("{}, {}: {}", x, y, x * 4_000_000 + y);
            }
            y += 1;
        }
    }
}
