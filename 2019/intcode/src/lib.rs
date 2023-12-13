use std::io::stdin;
use std::io::stdout;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::str::FromStr;
use std::io::Write as _;

pub type Program<O = Stdout> = ProgramT<O>;

pub trait OutputSender {
    type Receiver: OutputReceiver;

    fn send(&mut self, data: isize);
}

impl OutputSender for Stdout {
    type Receiver = ();

    fn send(&mut self, data: isize) {
        writeln!(self, "{}", data).unwrap();
    }
}

impl OutputSender for Sender<isize> {
    type Receiver = Receiver<isize>;

    fn send(&mut self, data: isize) {
        Sender::<isize>::send(self, data).unwrap();
    }
}

pub trait OutputReceiver {
    fn recv(&mut self) -> Option<isize>;
}

impl OutputReceiver for () {
    fn recv(&mut self) -> Option<isize> {
        None
    }
}

impl OutputReceiver for Receiver<isize> {
    fn recv(&mut self) -> Option<isize> {
        Receiver::<isize>::recv(self).ok()
    }
}

pub struct ProgramT<O: OutputSender> {
    pub data: Vec<isize>,
    pub input: Option<Receiver<isize>>,
    pub output: Option<O::Receiver>,
    pub sender: Option<O>,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_data(s, stdout(), Some(()))
    }
}

impl<O: OutputSender> ProgramT<O> {
    pub fn from_data(s: &str, sender: O, receiver: Option<O::Receiver>) -> Result<Self, String> {
        let data = s
            .split(',')
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect();
        Ok(Program {
            data,
            input: None,
            output: receiver,
            sender: Some(sender),
        })
    }

    pub fn value(&self) -> isize {
        self.data[0]
    }

    pub fn execute_with_input(&mut self, input: Vec<isize>, input_sender: Sender<isize>) {
        assert!(self.input.is_some(), "Must have an input channel");
        for elem in input {
            input_sender.send(elem).unwrap();
        }
        self.execute();
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
                let op = Op::Add(
                    self.data[i + 1],
                    self.data[i + 2],
                    self.data[i + 3] as usize,
                );
                i += 4;
                op
            } else if elem == 2 {
                let op = Op::Mult(
                    self.data[i + 1],
                    self.data[i + 2],
                    self.data[i + 3] as usize,
                );
                i += 4;
                op
            } else if elem == 3 {
                let op = Op::Input(self.data[i + 1] as usize);
                i += 2;
                op
            } else if elem == 4 {
                let op = Op::Output(self.data[i + 1]);
                i += 2;
                op
            } else if elem == 5 {
                let op = Op::Jtrue(self.data[i + 1], self.data[i + 2] as usize);
                i += 3;
                op
            } else if elem == 6 {
                let op = Op::Jfalse(self.data[i + 1], self.data[i + 2] as usize);
                i += 3;
                op
            } else if elem == 7 {
                let op = Op::Lt(
                    self.data[i + 1],
                    self.data[i + 2],
                    self.data[i + 3] as usize,
                );
                i += 4;
                op
            } else if elem == 8 {
                let op = Op::Eq(
                    self.data[i + 1],
                    self.data[i + 2],
                    self.data[i + 3] as usize,
                );
                i += 4;
                op
            } else {
                panic!("Unexpected token {} at {}", elem, i);
            };
            if let Some(inst) = self.eval_op(op, mask) {
                i = inst;
            }
        }
    }

    fn eval_op(&mut self, op: Op, mask: isize) -> Option<usize> {
        op.eval(
            &mut self.data,
            mask,
            self.input.as_mut(),
            self.sender.as_mut().unwrap(),
        )
    }

    pub fn output(&mut self) -> String {
        use std::fmt::Write as _;
        let mut output = String::new();
        self.sender = None;
        let receiver = self.output.as_mut().unwrap();
        while let Some(v) = receiver.recv() {
            writeln!(&mut output, "{}", v).unwrap();
        }
        output
    }

    pub fn print(&mut self) {
        print!("{}", self.output());
    }
}

enum Op {
    Add(isize, isize, usize),
    Mult(isize, isize, usize),
    Input(usize),
    Output(isize),
    Jtrue(isize, usize),
    Jfalse(isize, usize),
    Lt(isize, isize, usize),
    Eq(isize, isize, usize),
}

impl Op {
    fn eval<O: OutputSender>(
        &self,
        data: &mut [isize],
        mask: isize,
        input: Option<&mut Receiver<isize>>,
        output: &mut O,
    ) -> Option<usize> {
        match self {
            Self::Add(a, b, c) => self.binop(*a, *b, *c, data, |a, b| a + b, mask),
            Self::Mult(a, b, c) => self.binop(*a, *b, *c, data, |a, b| a * b, mask),
            Self::Input(l) => {
                let mut s = String::new();
                let value = match input {
                    Some(input) => input.recv().unwrap(),
                    None => {
                        stdin().read_line(&mut s).unwrap();
                        s.trim().parse().unwrap()
                    }
                };
                data[*l] = value;
                None
            }
            Self::Output(l) => {
                let l = if mask % 10 == 1 {
                    *l
                } else if mask % 10 == 0 {
                    *data.get(*l as usize).unwrap()
                } else {
                    unreachable!();
                };
                output.send(l);
                None
            }
            Self::Jtrue(a, b) => self.jump(*a, *b, data, |b| b != 0, mask),
            Self::Jfalse(a, b) => self.jump(*a, *b, data, |b| b == 0, mask),
            Self::Lt(a, b, c) => {
                self.binop(*a, *b, *c, data, |a, b| if a < b { 1 } else { 0 }, mask)
            }
            Self::Eq(a, b, c) => {
                self.binop(*a, *b, *c, data, |a, b| if a == b { 1 } else { 0 }, mask)
            }
        }
    }

    fn binop(
        &self,
        a: isize,
        b: isize,
        c: usize,
        data: &mut [isize],
        f: impl FnOnce(isize, isize) -> isize,
        mask: isize,
    ) -> Option<usize> {
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

    fn jump(
        &self,
        a: isize,
        b: usize,
        data: &mut [isize],
        f: impl FnOnce(isize) -> bool,
        mask: isize,
    ) -> Option<usize> {
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
            *data.get(b).unwrap() as usize
        } else {
            unreachable!()
        };
        if f(a) {
            Some(b)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // Day 1 //

    // add and mult tests
    #[test]
    fn a() {
        let mut program = Program::from_str("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
        program.execute();
        assert_eq!(program.value(), 3500)
    }

    #[test]
    fn b() {
        let mut program = Program::from_str("1,0,0,0,99").unwrap();
        program.execute();
        assert_eq!(program.value(), 2)
    }

    #[test]
    fn c() {
        let mut program = Program::from_str("2,3,0,3,99").unwrap();
        program.execute();
        assert_eq!(program.data[3], 6)
    }

    #[test]
    fn d() {
        let mut program = Program::from_str("2,4,4,5,99,0").unwrap();
        program.execute();
        assert_eq!(program.data[5], 9801)
    }

    #[test]
    fn e() {
        let mut program = Program::from_str("1,1,1,4,99,5,6,0,99").unwrap();
        program.execute();
        assert_eq!(program.data[0], 30)
    }

    // DAY 5 //

    // mode tests
    #[test]
    fn f() {
        let mut program = Program::from_str("1002,4,3,4,33").unwrap();
        program.execute();
        assert_eq!(program.data[4], 99)
    }

    #[test]
    fn g() {
        let mut program = Program::from_str("1101,100,-1,4,0").unwrap();
        program.execute();
        assert_eq!(program.data[4], 99)
    }

    // io tests

    #[test]
    fn h() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,9,8,9,10,9,4,9,99,-1,8", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![8], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }

    #[test]
    fn i() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,9,8,9,10,9,4,9,99,-1,8", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn j() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,9,7,9,10,9,4,9,99,-1,8", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![8], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn k() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,9,7,9,10,9,4,9,99,-1,8", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }

    #[test]
    fn l() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1108,-1,8,3,4,3,99", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![8], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }

    #[test]
    fn m() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1108,-1,8,3,4,3,99", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn n() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1107,-1,8,3,4,3,99", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![8], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn o() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1107,-1,8,3,4,3,99", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }

    // jump tests
    #[test]
    fn p() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![0], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn q() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }

    #[test]
    fn r() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![0], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "0");
    }

    #[test]
    fn s() {
        
        let (s, r) = channel(); let mut program = ProgramT::from_data("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", s, Some(r)).unwrap();
        let (s, r) = channel();
        program.input = Some(r);
        program.execute_with_input(vec![7], s);
        
        let s = program.output();
        assert_eq!(s.trim(), "1");
    }
}