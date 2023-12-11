use std::str::FromStr;
use std::io::stdin;

#[derive(Clone)]
pub struct Program {
    pub data: Vec<isize>,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(',').map(|s| s.trim().parse::<isize>().unwrap()).collect();
        Ok(Program { data })
    }
}

impl Program {
    pub fn value(&self) -> isize {
        self.data[0]
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.data.len() {
            let elem = self.data[i];
            let mask = elem / 100;
            let elem = elem % 100;
            let op = if elem == 99 {
                break;
            } else if elem == 1 {
                let op = Op::Add(self.data[i+1], self.data[i+2], self.data[i+3] as usize);
                i += 4;
                op
            } else if elem == 2 {
                let op = Op::Mult(self.data[i+1], self.data[i+2], self.data[i+3] as usize);
                i += 4;
                op
            } else if elem == 3 {
                let op = Op::Input(self.data[i+1] as usize);
                i += 2;
                op
            } else if elem == 4 {
                let op = Op::Output(self.data[i+1] as usize);
                i += 2;
                op
            }else if elem == 5 {
                let op = Op::Jtrue(self.data[i+1], self.data[i+2] as usize);
                i += 2;
                op
            } else if elem == 6 {
                let op = Op::Jfalse(self.data[i+1], self.data[i+2] as usize);
                i += 2;
                op
            } else if elem == 7 {
                let op = Op::Lt(self.data[i+1], self.data[i+2], self.data[i+3] as usize);
                i += 3;
                op
            } else if elem == 8 {
                let op = Op::Eq(self.data[i+1], self.data[i+2], self.data[i+3] as usize);
                i += 3;
                op
            } else {
                panic!("Unexpected token {} at {}", elem, i);
            };
            if let Some(inst) = op.eval(&mut self.data, mask) {
                i = inst;
            }
        }
    }
}

enum Op {
    Add(isize, isize, usize),
    Mult(isize, isize, usize),
    Input(usize),
    Output(usize),
    Jtrue(isize, usize),
    Jfalse(isize, usize),
    Lt(isize, isize, usize),
    Eq(isize, isize, usize),
}

impl Op {
    fn eval(&self, data: &mut [isize], mask: isize) -> Option<usize> {
        match self {
            Self::Add(a, b, c) => self.binop(*a, *b, *c, data, |a, b| a + b, mask),
            Self::Mult(a, b, c) => self.binop(*a, *b, *c, data, |a, b| a * b, mask),
            Self::Input(l) => {
                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                println!("Read {s}");
                let value = s.trim().parse().unwrap();
                data[*l as usize] = value;
                None
            },
            Self::Output(l) => {
                println!("{}", data[*l as usize]);
                None
            },
            Self::Jtrue(a, b) => self.jump(*a, *b, data, |b| b != 0, mask),
            Self::Jfalse(a, b) => self.jump(*a, *b, data, |b| b == 0, mask),
            Self::Lt(a, b, c) => self.binop(*a, *b, *c, data, |a, b| if a < b { 1 } else { 0 }, mask),
            Self::Eq(a, b, c) => self.binop(*a, *b, *c, data, |a, b| if a == b { 1 } else { 0 }, mask),
        } 
    }

    fn binop(&self, a: isize, b: isize, c: usize, data: &mut [isize], f: impl FnOnce(isize, isize) -> isize, mask: isize) -> Option<usize> {
        let a = if mask % 10 == 1 {
            a
        } else if mask % 10 == 0 {
            *data.get(a as usize).unwrap()
        } else {
            unreachable!();
        };
        let mask = mask / 10;
        let b = if mask % 10 == 1 {
            b
        } else if mask % 10 == 0 {
            *data.get(b as usize).unwrap()
        } else {
            unreachable!()
        };
        let result = f(a, b);
        data[c] = result;
        None
    }

    fn jump(&self, a: isize, b: usize, data: &mut [isize], f: impl FnOnce(isize) -> bool, mask: isize) -> Option<usize> {
        let a = if mask % 10 == 1 {
            a
        } else if mask % 10 == 0 {
            *data.get(a as usize).unwrap()
        } else {
            unreachable!();
        };
        if f(a) {
            Some(b)
        } else {
            None
        }
    }
}