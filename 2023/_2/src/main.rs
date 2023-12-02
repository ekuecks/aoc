use std::io::stdin;

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    fn valid(&self, red: usize, green: usize, blue: usize) -> bool {
        self.reveals.iter().all(|reveal| reveal.valid(red, green, blue))
    }

    fn power(&self) -> usize {
        let red = self.reveals.iter().map(|r| r.red).max().unwrap_or(0);
        let green = self.reveals.iter().map(|r| r.green).max().unwrap_or(0);
        let blue = self.reveals.iter().map(|r| r.blue).max().unwrap_or(0);
        red * blue * green
    }
}

#[derive(Debug, Default)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl Reveal {
    fn valid(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}


fn parse_line(line: String) -> Game {
    let parts: Vec<_> = line.split(":").collect();
    assert!(parts.len() == 2);
    let id: usize = parts[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
    let mut reveals = Vec::new();
    for s in parts[1].split(";") {
        let mut reveal = Reveal::default();
        let s = s.trim();
        for c in s.split(", ") {
            let c = c.trim();
            let parts: Vec<_> = c.split(" ").collect();
            let color = parts[1];
            let num: usize = parts[0].parse().unwrap();
            if color == "green" {
                reveal.green = num;
            } else if color == "red" {
                reveal.red = num;
            } else if color == "blue" {
                reveal.blue = num;
            }
        }
        reveals.push(reveal)
    }
    Game { id, reveals }
}

fn main() {
    let input = stdin();
    let mut sum: usize = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let game = parse_line(line);
        //if game.valid(12, 13, 14) {
        //    sum += game.id
        //}
        sum += game.power();
    }
    dbg!(sum);
}