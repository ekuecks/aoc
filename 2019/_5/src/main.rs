use std::io::stdin;
use intcode::Program;
use std::str::FromStr;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    dbg!(&filename);
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let buf: Vec<_> = buf.into_iter().enumerate().filter(|(i, c)| i % 2 == 0).map(|(i, c)| c).skip(1).collect();
    let buf = String::from_utf8(buf).unwrap();
    let lines: Vec<_> = buf.split_ascii_whitespace().collect();
    let mut inputs: Vec<_> = lines.into_iter().map(|s| Program::from_str(s.trim()).unwrap()).collect();
    let program = &mut inputs[0];
    program.execute();
}
