use intcode::ProgramT;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::mpsc::Sender;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let buf: Vec<_> = buf
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c)
        .skip(1)
        .collect();
    let buf = String::from_utf8(buf).unwrap();
    let lines: Vec<_> = buf.split_ascii_whitespace().collect();
    let mut programs
    let mut inputs: Vec<_> = lines
        .iter()
        .map(|s| ProgramT::from_data(s.trim(), Vec::<u8>::new()).unwrap())
        .collect();
    let program = &mut inputs[0];
    let mut part1 = -99999999;
    for a in 5..=9 {
        program.input = Some(vec![0, a]);
        program.output = Vec::new();
        program.execute();
        let output = String::from_utf8(program.output.clone()).unwrap();
        let input = output.trim().parse().unwrap();
        for b in 5..=9 {
            if a == b {
                continue;
            }
            program.input = Some(vec![input, b]);
            program.output = Vec::new();
            program.execute();
            let output = String::from_utf8(program.output.clone()).unwrap();
            let input = output.trim().parse().unwrap();
            for c in 5..=9 {
                if a == c || b == c {
                    continue;
                }
                program.input = Some(vec![input, c]);
                program.output = Vec::new();
                program.execute();
                let output = String::from_utf8(program.output.clone()).unwrap();
                let input = output.trim().parse().unwrap();
                for d in 5..=9 {
                    if a == d || b== d || c == d {
                        continue;
                    }
                    program.input = Some(vec![input, d]);
                    program.output = Vec::new();
                    program.execute();
                    let output = String::from_utf8(program.output.clone()).unwrap();
                    let input = output.trim().parse().unwrap();
                    for e in 5..=9 {
                        if a == e || b== e || c == e || d == e {
                            continue;
                        }
                        program.input = Some(vec![input, e]);
                        program.output = Vec::new();
                        program.execute();
                        let output = String::from_utf8(program.output.clone()).unwrap();
                        let input = output.trim().parse().unwrap();
                        part1 = part1.max(input);
                    }
                }
            }
        }
    }
    dbg!(part1);
}


fn run(program: Program) {

}