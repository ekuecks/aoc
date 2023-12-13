use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = instruction.split(" ").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid instruction: {}", instruction));
        }
        let val: i64 = parts[1]
            .parse()
            .map_err(|_e| format!("Invalid int: {}", parts[1]))?;
        match parts[0] {
            "nop" => Ok(Self::NOP(val)),
            "acc" => Ok(Self::ACC(val)),
            "jmp" => Ok(Self::JMP(val)),
            _ => Err(format!("Invalid instruction: {}", instruction)),
        }
    }
}

fn main() -> Result<(), String> {
    let input = stdin();
    let mut instructions = Vec::new();
    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => instructions.push(Instruction::from_str(buf.trim())?),
            e => return Err(format!("Failed to parse input: {:?}", e)),
        }
    }
    for i in 0..instructions.len() {
        match instructions[i] {
            Instruction::ACC(_) => continue,
            Instruction::NOP(val) => {
                instructions[i] = Instruction::JMP(val);
                if let Ok(accum) = execute_instructions(&instructions) {
                    println!("Found valid value: {} by changing: {}", accum, i);
                }
                instructions[i] = Instruction::NOP(val);
            }
            Instruction::JMP(val) => {
                instructions[i] = Instruction::NOP(val);
                if let Ok(accum) = execute_instructions(&instructions) {
                    println!("Found valid value: {} by changing: {}", accum, i);
                }
                instructions[i] = Instruction::JMP(val);
            }
        }
    }
    Ok(())
}

fn execute_instructions(instructions: &[Instruction]) -> Result<i64, String> {
    let mut accum = 0;
    let mut index: i64 = 0;
    let mut already_ran = HashSet::new();
    while index >= 0 && (index as usize) < instructions.len() {
        if already_ran.contains(&index) {
            return Err("Infinite loop".to_owned());
        }
        already_ran.insert(index);
        match instructions[index as usize] {
            Instruction::ACC(to_add) => accum += to_add,
            Instruction::JMP(offset) => index += offset - 1,
            _ => {}
        }
        index += 1;
    }
    if index < 0 {
        Err("Didn't run last instruction".to_owned())
    } else {
        Ok(accum)
    }
}
