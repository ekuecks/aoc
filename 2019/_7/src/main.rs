use intcode::ProgramT;
use std::fs::File;
use std::io::Read;

use std::sync::mpsc::{channel};
use std::thread;

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
    let mut part2 = -99999999;
    for a in 5..=9 {
        for b in 5..=9 {
            if a == b {
                continue;
            }
            for c in 5..=9 {
                if a == c || b == c {
                    continue;
                }
                for d in 5..=9 {
                    if a == d || b == d || c == d {
                        continue;
                    }
                    for e in 5..=9 {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }
                        let (s, r) = channel::<isize>();
                        let mut programa = ProgramT::from_data(input, s, Some(r)).unwrap();
                        let (s, r) = channel::<isize>();
                        let mut programb = ProgramT::from_data(input, s, Some(r)).unwrap();
                        programb.input = programa.output.take();
                        let _ = programa.sender.as_mut().unwrap().send(b);
                        let (s, r) = channel::<isize>();
                        let mut programc = ProgramT::from_data(input, s, Some(r)).unwrap();
                        programc.input = programb.output.take();
                        let _ = programb.sender.as_mut().unwrap().send(c);
                        let (s, r) = channel::<isize>();
                        let mut programd = ProgramT::from_data(input, s, Some(r)).unwrap();
                        programd.input = programc.output.take();
                        let _ = programc.sender.as_mut().unwrap().send(d);
                        let (s, r) = channel::<isize>();
                        let mut programe = ProgramT::from_data(input, s, Some(r)).unwrap();
                        programe.input = programd.output.take();
                        let _ = programd.sender.as_mut().unwrap().send(e);
                        programa.input = programe.output.take();
                        let _ = programe.sender.as_mut().unwrap().send(a);
                        let _ = programe.sender.as_mut().unwrap().send(0);
                        let mut handles = Vec::new();
                        handles.push(thread::spawn(move || {
                            programa.execute();
                            programa.input.as_mut().unwrap().recv()
                        }));
                        handles.push(thread::spawn(move || {
                            programb.execute();
                            programb.input.as_mut().unwrap().recv()
                        }));
                        handles.push(thread::spawn(move || {
                            programc.execute();
                            programc.input.as_mut().unwrap().recv()
                        }));
                        handles.push(thread::spawn(move || {
                            programd.execute();
                            programd.input.as_mut().unwrap().recv()
                        }));
                        handles.push(thread::spawn(move || {
                            programe.execute();
                            programe.input.as_mut().unwrap().recv()
                        }));
                        for (i, handle) in handles.into_iter().enumerate() {
                            let output = handle.join().unwrap();
                            if i == 0 {
                                part2 = part2.max(output.unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
    dbg!(part2);
}
