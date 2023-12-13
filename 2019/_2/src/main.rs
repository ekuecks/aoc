use std::io::stdin;
use intcode::Program;
use std::str::FromStr;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let input = lines[0].trim();
    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut program = Program::from_str(input).unwrap();
            program.data[1] = noun;
            program.data[2] = verb;
            program.execute();
            if program.value() == 19690720 {
                dbg!(100 * noun + verb);
                break 'outer;
            }
        }
    }
}
