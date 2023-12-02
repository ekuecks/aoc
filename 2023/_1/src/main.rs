use std::io::stdin;

fn parse(line: String) -> usize {
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() {
        let mut c = chars[i];
        if i + 3 < chars.len() && chars[i..i+4] == ['z', 'e', 'r', 'o'] {
            c = '0';
        }
        if i + 2 < chars.len() && chars[i..i+3] == ['o', 'n', 'e'] {
            c = '1';
        }
        if i + 2 < chars.len() && chars[i..i+3] == ['t', 'w', 'o'] {
            c = '2';
        }
        if i + 4 < chars.len() && chars[i..i+5] == ['t', 'h', 'r', 'e', 'e'] {
            c = '3';
        }
        if i + 3 < chars.len() && chars[i..i+4] == ['f', 'o', 'u', 'r'] {
            c = '4';
        }
        if i + 3 < chars.len() && chars[i..i+4] == ['f', 'i', 'v', 'e'] {
            c = '5';
        }
        if i + 2 < chars.len() && chars[i..i+3] == ['s', 'i', 'x'] {
            c = '6';
        }
        if i + 4 < chars.len() && chars[i..i+5] == ['s', 'e', 'v', 'e', 'n'] {
            c = '7';
        }
        if i + 4 < chars.len() && chars[i..i+5] == ['e', 'i', 'g', 'h', 't'] {
            c = '8';
        }
        if i + 3 < chars.len() && chars[i..i+4] == ['n', 'i', 'n', 'e'] {
            c = '9';
        }
        if c as usize >= '0' as usize && c as usize <= '9' as usize {
            if first.is_none() {
                first = Some(c as usize - '0' as usize);
            }
            last = Some(c as usize - '0' as usize);
        }
    }
    first.unwrap() * 10 + last.unwrap()
}

fn main() {
    let input = stdin();
    let mut sum: usize = 0;
    for line in input.lines() {
        let line = line.unwrap();
        sum += parse(line);
    }
    dbg!(sum);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(parse("zero".to_string()), 0);
        assert_eq!(parse("one".to_string()), 11);
        assert_eq!(parse("two".to_string()), 22);
        assert_eq!(parse("three".to_string()), 33);
        assert_eq!(parse("four".to_string()), 44);
        assert_eq!(parse("five".to_string()), 55);
        assert_eq!(parse("six".to_string()), 66);
        assert_eq!(parse("seven".to_string()), 77);
        assert_eq!(parse("eight".to_string()), 88);
        assert_eq!(parse("nine".to_string()), 99);
    }
}