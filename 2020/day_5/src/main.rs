use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

struct Seat {
    pub row: u64,
    pub col: u64,
}

impl Seat {
    fn id(&self) -> u64 {
        self.row * 8 + self.col
    }
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(identifier: &str) -> Result<Self, Self::Err> {
        if identifier.len() != 10 {
            return Err(format!(
                "Invalid identifier: {}. Expected 10 chars",
                identifier
            ));
        }
        let row_id = &identifier[0..7];
        let mut front = 0;
        let mut back = 127;
        for c in row_id.chars() {
            match c {
                'F' => back = (front + back + 1) / 2 - 1,
                'B' => front = (front + back + 1) / 2,
                _ => return Err(format!("Invalid row identifier '{}'", c)),
            }
        }
        let col_id = &identifier[7..10];
        let mut left = 0;
        let mut right = 7;
        for c in col_id.chars() {
            match c {
                'L' => right = (left + right + 1) / 2 - 1,
                'R' => left = (left + right + 1) / 2,
                _ => return Err(format!("Invalid col identifier '{}'", c)),
            }
        }
        assert_eq!(front, back);
        assert_eq!(left, right);
        Ok(Self {
            row: front,
            col: left,
        })
    }
}

fn main() -> Result<(), String> {
    let input = stdin();
    let mut all_seats = (0..893).collect::<HashSet<u64>>();
    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let seat = Seat::from_str(&buf.trim())?;
                all_seats.remove(&seat.id());
            }
            _ => panic!("Failed to read stdin"),
        }
    }
    println!("{:?}", all_seats.into_iter().max());
    Ok(())
}
