use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;
use std::ops::Range;
use std::str::FromStr;

struct Input {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1];
        let mut vals = [0; 4];
        for (i, s) in s.split(',').enumerate() {
            let (_, r) = s.split_once('=').unwrap();
            vals[i] = r.parse::<usize>().unwrap();
        }
        Ok(Self {
            x: vals[0],
            m: vals[1],
            a: vals[2],
            s: vals[3],
        })
    }
}

#[derive(Clone, Debug)]
struct Multi {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('{').unwrap();
        let r = &r[0..r.len() - 1];
        let mut rules = Vec::new();
        for s in r.split(',') {
            rules.push(Rule::from_str(s).unwrap());
        }
        Ok(Workflow {
            name: l.to_string(),
            rules,
        })
    }
}

enum Rule {
    Direct(String),
    Send(SendRule, String),
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = match s.split_once(':') {
            Some((l, r)) => (l, r),
            _ => return Ok(Rule::Direct(s.to_string())),
        };
        let cmp = if s.contains('>') { Cmp::Gt } else { Cmp::Lt };
        let attr = &l[0..1];
        let val = &l[2..].parse::<usize>().unwrap();
        Ok(Rule::Send(
            SendRule {
                attr: attr.to_string(),
                val: *val,
                cmp,
            },
            r.to_string(),
        ))
    }
}

#[derive(Debug)]
struct SendRule {
    attr: String,
    val: usize,
    cmp: Cmp,
}

impl SendRule {
    fn m_matches(&self, m: &Multi) -> (Option<Multi>, Option<Multi>) {
        let v = match &*self.attr {
            "x" => &m.x,
            "m" => &m.m,
            "a" => &m.a,
            "s" => &m.s,
            _ => unreachable!(),
        }
        .clone();
        let (pass, fail) = match self.cmp {
            Cmp::Lt => {
                if v.contains(&self.val) {
                    (Some(v.start..self.val), Some(self.val..v.end))
                } else if v.start < self.val {
                    (Some(v), None)
                } else {
                    (None, Some(v))
                }
            }
            Cmp::Gt => {
                if v.contains(&self.val) {
                    (Some((self.val + 1)..v.end), Some(v.start..(self.val + 1)))
                } else if v.start > self.val {
                    (Some(v), None)
                } else {
                    (None, Some(v))
                }
            }
        };
        let mpass = pass.map(|r| {
            let mut mpass = m.clone();
            match &*self.attr {
                "x" => mpass.x = r,
                "m" => mpass.m = r,
                "a" => mpass.a = r,
                "s" => mpass.s = r,
                _ => unreachable!(),
            };
            mpass
        });
        let mfail = fail.map(|r| {
            let mut mfail = m.clone();
            match &*self.attr {
                "x" => mfail.x = r,
                "m" => mfail.m = r,
                "a" => mfail.a = r,
                "s" => mfail.s = r,
                _ => unreachable!(),
            };
            mfail
        });
        (mpass, mfail)
    }

    fn matches(&self, w: &Input) -> bool {
        let v = match &*self.attr {
            "x" => w.x,
            "m" => w.m,
            "a" => w.a,
            "s" => w.s,
            _ => unreachable!(),
        };
        match self.cmp {
            Cmp::Lt => v < self.val,
            Cmp::Gt => v > self.val,
        }
    }
}

#[derive(Debug)]
enum Cmp {
    Lt,
    Gt,
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut inputs: Vec<Input> = Vec::new();
    let mut rules = HashMap::new();
    let mut in_rules = true;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            in_rules = false;
            continue;
        }
        if in_rules {
            let rule = Workflow::from_str(line).unwrap();
            rules.insert(rule.name.clone(), rule);
        } else {
            inputs.push(Input::from_str(line).unwrap());
        }
    }
    let mut rejects = HashSet::new();
    let mut accepts = HashSet::new();
    for (i, input) in inputs.iter().enumerate() {
        let mut rule_name = "in".to_string();
        'outer: loop {
            let rule = rules.get(&rule_name).unwrap();
            for rule in &rule.rules {
                match rule {
                    Rule::Direct(r) => {
                        if r == "A" {
                            accepts.insert(i);
                            break 'outer;
                        } else if r == "R" {
                            rejects.insert(i);
                            break 'outer;
                        } else {
                            rule_name = r.clone();
                            break;
                        }
                    }
                    Rule::Send(send, loc) => {
                        if send.matches(input) {
                            rule_name = loc.clone();
                            if loc == "A" {
                                accepts.insert(i);
                                break 'outer;
                            } else if loc == "R" {
                                rejects.insert(i);
                                break 'outer;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    let mut part1 = 0;
    for a in accepts {
        let a = inputs.get(a).unwrap();
        part1 += a.x + a.m + a.a + a.s;
    }
    dbg!(part1);
    let mut part2: usize = 0;
    let mut q = VecDeque::new();
    q.push_back((
        Multi {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        "in".to_string(),
    ));
    while let Some((mut m, mut rule_name)) = q.pop_front() {
        let rule = rules.get(&rule_name).unwrap();
        for rule in &rule.rules {
            match rule {
                Rule::Direct(r) => {
                    if r == "A" {
                        let x = m.x.len();
                        let ms = m.m.len();
                        let a = m.a.len();
                        let s = m.s.len();
                        let v: usize = x * ms * a * s;
                        part2 += v;
                        break;
                    } else if r == "R" {
                        break;
                    } else {
                        q.push_back((m, r.clone()));
                        break;
                    }
                }
                Rule::Send(send, loc) => {
                    let (pass, fail) = send.m_matches(&m);
                    if let Some(mpass) = pass {
                        rule_name = loc.clone();
                        if loc == "A" {
                            let x = mpass.x.len();
                            let ms = mpass.m.len();
                            let a = mpass.a.len();
                            let s = mpass.s.len();
                            let v: usize = x * ms * a * s;
                            part2 += v;
                        } else if loc == "R" {
                        } else {
                            q.push_back((mpass, rule_name))
                        }
                    }
                    if let Some(mfail) = fail {
                        m = mfail;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    dbg!(part2);
}
