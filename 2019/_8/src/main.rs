use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    assert_eq!(lines.len(), 1);
    let ncols = 25;
    let nrows = 6;
    let mut i = 0;
    let mut ans = 0;
    let mut zeros = usize::MAX;
    let mut this = 0;
    let mut ones = 0;
    let mut twos = 0;
    for line in &lines {
        let line = line.trim();
        for c in line.chars() {
            i += 1;
            if c == '0' {
                this += 1;
            } else if c == '1' {
                ones += 1;
            } else if c == '2' {
                twos += 1;
            }
            if i == ncols * nrows {
                if this < zeros {
                    ans = ones * twos;
                    zeros = this;
                }
                ones = 0;
                twos = 0;
                this = 0;
                i = 0;
            }
        }
    }
    assert_eq!(i, 0);
    dbg!(ans);

    #[derive(Clone, Copy)]
    enum Color {
        B,
        W,
        T,
    }

    struct Layer {
        pixels: Vec<Vec<Color>>,
    }
    let mut x = 0;
    let mut y = 0;
    let mut layers = Vec::new();
    let mut row = Vec::new();
    let mut rows = Vec::new();
    for line in lines {
        let line = line.trim();
        for c in line.chars() {
            x += 1;
            let color = match c {
                '0' => Color::B,
                '1' => Color::W,
                '2' => Color::T,
                _ => unreachable!()
            };
            row.push(color);
            if x == ncols {
                rows.push(row);
                row = Vec::new();
                x = 0;
                y += 1;
                if y == nrows {
                    layers.push(Layer { pixels: rows });
                    y = 0;
                    rows = Vec::new();
                }
            }
        }
    }

    for i in 0..nrows {
        for j in 0..ncols {
            let mut printed = false;
            for layer in layers.iter() {
                match layer.pixels[i][j] {
                    Color::B => {
                        print!(" ");
                        printed = true;
                        break;
                    }
                    Color::W => {
                        print!("#");
                        printed = true;
                        break;
                    }
                    _ => {}
                }
            }
                if !printed {
                    print!(".");
                }
        }
        println!();
    }
}