use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::stdin;

#[derive(Clone, Copy)]
enum Cube {
    Active,
    Inactive,
}

impl TryFrom<char> for Cube {
    type Error = String;

    fn try_from(s: char) -> Result<Self, String> {
        match s {
            '.' => Ok(Self::Inactive),
            '#' => Ok(Self::Active),
            _ => Err(format!("Invalid cube: {}", s)),
        }
    }
}

fn main() {
    let input = stdin();
    let mut map = HashMap::new();
    let mut row = 0;
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.trim().is_empty() {
            break;
        }
        let cubes = buf.trim().chars().map(|s| Cube::try_from(s).unwrap());
        for (col, cube) in cubes.into_iter().enumerate() {
            map.insert((row as isize, col as isize, 0, 0), cube);
        }
        row += 1;
    }
    for _ in 0..6 {
        map = iterate(map);
    }
    let count = map.iter().fold(0, |accum, (_point, cube)| match cube {
        Cube::Active => accum + 1,
        _ => accum,
    });
    println!("{}", count);
}

fn iterate(
    map: HashMap<(isize, isize, isize, isize), Cube>,
) -> HashMap<(isize, isize, isize, isize), Cube> {
    let mut result = HashMap::new();
    let mut min_x = 100;
    let mut min_y = 100;
    let mut min_z = 100;
    let mut min_w = 100;
    let mut max_x = -100;
    let mut max_y = -100;
    let mut max_z = -100;
    let mut max_w = -100;
    for ((x, y, z, w), _) in map.iter() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *z < min_z {
            min_z = *z;
        }
        if *w < min_w {
            min_w = *w;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
        if *z > max_z {
            max_z = *z;
        }
        if *w > max_w {
            max_w = *w;
        }
    }
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    let cube = map.get(&(x, y, z, w)).cloned().unwrap_or(Cube::Inactive);
                    let mut active = 0;
                    for i in &[-1, 0, 1] {
                        for j in &[-1, 0, 1] {
                            for k in &[-1, 0, 1] {
                                for l in &[-1, 0, 1] {
                                    if i == j && i == k && i == l && *i == 0 {
                                        continue;
                                    }
                                    let other = map
                                        .get(&(x + *i, y + *j, z + *k, w + *l))
                                        .cloned()
                                        .unwrap_or(Cube::Inactive);
                                    if matches!(other, Cube::Active) {
                                        active += 1;
                                    }
                                }
                            }
                        }
                    }
                    match cube {
                        Cube::Active => {
                            if active == 2 || active == 3 {
                                result.insert((x, y, z, w), Cube::Active);
                            }
                        }
                        Cube::Inactive => {
                            if active == 3 {
                                result.insert((x, y, z, w), Cube::Active);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}
