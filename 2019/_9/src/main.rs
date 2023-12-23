use intcode::Program;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

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
    let input = lines[0].trim();
    let mut program = Program::from_str(input).unwrap();
    program.execute();
}
