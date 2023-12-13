use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::stdin;

#[derive(Debug)]
enum Operator {
    Times,
    Plus,
    Open,
    Close,
}

impl TryFrom<char> for Operator {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '*' => Ok(Self::Times),
            '+' => Ok(Self::Plus),
            '(' => Ok(Self::Open),
            ')' => Ok(Self::Close),
            c => Err(format!("Invalid operator: '{}'", c)),
        }
    }
}

#[derive(Default)]
struct Evaluator {
    operators: VecDeque<Operator>,
    operands: VecDeque<i64>,
}

impl Evaluator {
    fn push_operator(&mut self, operator: Operator) -> Result<(), String> {
        match operator {
            Operator::Close => {
                let total = self
                    .operands
                    .pop_back()
                    .ok_or_else(|| "Empty operands for ')'".to_owned())?;
                #[cfg(feature = "part2")]
                let mut total = total;
                loop {
                    match self.operators.pop_back() {
                        Some(Operator::Open) => {
                            return self.push_operand(total);
                        }
                        #[cfg(feature = "part2")]
                        Some(Operator::Times) => {
                            let other = self
                                .operands
                                .pop_back()
                                .ok_or_else(|| "Empty operands for ')'".to_owned())?;
                            total *= other;
                        }
                        Some(o) => {
                            return Err(format!("Unexpected operator {:?}", o));
                        }
                        None => return Err("Unexpected '('".to_owned()),
                    }
                }
            }
            _ => {
                self.operators.push_back(operator);
                Ok(())
            }
        }
    }

    fn push_operand(&mut self, operand: i64) -> Result<(), String> {
        match self.operators.back() {
            Some(Operator::Plus) => {
                let _ = self.operators.pop_back();
                match self.operands.pop_back() {
                    Some(other) => self.operands.push_back(operand + other),
                    None => return Err("Too few operands".to_owned()),
                }
            }
            #[cfg(feature = "part1")]
            Some(Operator::Times) => {
                let _ = self.operators.pop_back();
                match self.operands.pop_back() {
                    Some(other) => self.operands.push_back(operand * other),
                    None => return Err("Too few operands".to_owned()),
                }
            }
            _ => self.operands.push_back(operand),
        }
        Ok(())
    }

    fn push_token(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::Operator(operator) => self.push_operator(operator),
            Token::Const(operand) => self.push_operand(operand),
        }
    }

    fn evaluate(&mut self) -> Result<i64, String> {
        let mut total = match self.operands.pop_back() {
            Some(initial) => initial,
            None => return Err("No operands".to_owned()),
        };
        while !self.operands.is_empty() {
            match self.operators.pop_back() {
                Some(Operator::Times) => {
                    total *= self
                        .operands
                        .pop_back()
                        .ok_or_else(|| "Too few operators".to_owned())?;
                }
                other => return Err(format!("Unexpected operator: {:?}", other)),
            }
        }
        Ok(total)
    }
}

enum Token {
    Operator(Operator),
    Const(i64),
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut buf = String::new();
    let mut tokens = Vec::new();
    for c in s.trim().chars() {
        match c {
            ' ' => {
                if !buf.is_empty() {
                    tokens.push(Token::Const(
                        buf.parse().map_err(|e| format!("Invalid int: {:?}", e))?,
                    ));
                    buf = String::new();
                }
            }
            '+' | '*' | '(' | ')' => {
                if !buf.is_empty() {
                    tokens.push(Token::Const(
                        buf.parse().map_err(|e| format!("Invalid int: {:?}", e))?,
                    ));
                }
                tokens.push(Token::Operator(Operator::try_from(c)?));
                buf = String::new();
            }
            _ => {
                buf.push(c);
            }
        }
    }
    if !buf.is_empty() {
        tokens.push(Token::Const(
            buf.parse().map_err(|e| format!("Invalid int: {:?}", e))?,
        ));
    }
    Ok(tokens)
}

fn main() -> Result<(), String> {
    let input = stdin();
    let mut sum = 0;
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.trim().is_empty() {
            break;
        }
        let tokens = tokenize(buf.trim())?;
        let mut evaluator = Evaluator::default();
        for token in tokens {
            evaluator.push_token(token)?;
        }
        let next = evaluator.evaluate()?;
        sum += next;
    }
    println!("{}", sum);
    Ok(())
}

// Janky way for part 1
// fn parse(s: &str) -> (i64, &str) {
//     let s = s.trim();
//     let (mut total, mut rest) = if s.starts_with("(") {
//         parse(&s[1..s.len()])
//     } else {
//         let parts: Vec<_> = s.splitn(2, " ").collect();
//         let first = parts[0];
//         match first.find(")") {
//             Some(index) => return (first[0..index].parse().unwrap(), &s[index + 1..s.len()]),
//             None => {
//                 let left = first.parse().unwrap();
//                 if parts.len() == 1 {
//                     return (left, "");
//                 }
//                 (left, parts[1])
//             }
//         }
//     };
//     let mut found_close = false;
//     while !rest.is_empty() {
//         let trimmed = rest.trim();
//         if trimmed.starts_with(")") {
//             return (total, &trimmed[1..trimmed.len()]);
//         }
//         let parts: Vec<_> = trimmed.splitn(3, " ").collect();
//         let num_str = parts[1];
//         let (right, next_rest) = if num_str.starts_with("(") {
//             parse(&trimmed[3..trimmed.len()])
//         } else {
//             match num_str.find(")") {
//                 Some(index) => {
//                     found_close = true;
//                     (
//                         num_str[0..index].parse().unwrap(),
//                         &trimmed[2 + index + 1..trimmed.len()],
//                     )
//                 }
//                 None => {
//                     let right = num_str.parse().unwrap();
//                     if parts.len() == 2 {
//                         (right, "")
//                     } else {
//                         (right, parts[2])
//                     }
//                 }
//             }
//         };
//         rest = next_rest;
//         match parts[0] {
//             "+" => {
//                 total += right;
//             }
//             "*" => {
//                 total *= right;
//             }
//             _ => panic!(format!("Invalid operator {}", parts[0])),
//         }
//         if found_close {
//             return (total, rest);
//         }
//     }
//     (total, rest)
// }
