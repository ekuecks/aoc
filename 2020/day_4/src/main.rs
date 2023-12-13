use regex::Regex;
use std::collections::HashSet;
use std::io::stdin;

fn process_entry(entry: &str) -> bool {
    let mut fields = HashSet::new();
    fields.insert("byr");
    fields.insert("iyr");
    fields.insert("eyr");
    fields.insert("hgt");
    fields.insert("hcl");
    fields.insert("ecl");
    fields.insert("pid");
    for segment in entry.split(" ") {
        let mut parts = segment.split(":");
        if let Some(key) = parts.next() {
            fields.remove(key);
            let val = parts.next().unwrap();
            match key {
                "byr" => {
                    let val: u64 = val.parse().unwrap();
                    if val < 1920 || val > 2002 {
                        return false;
                    }
                }
                "iyr" => {
                    let val: u64 = val.parse().unwrap();
                    if val < 2010 || val > 2020 {
                        return false;
                    }
                }
                "eyr" => {
                    let val: u64 = val.parse().unwrap();
                    if val < 2020 || val > 2030 {
                        return false;
                    }
                }
                "hgt" => {
                    if val.contains("cm") {
                        if val.len() != 5 {
                            return false;
                        }
                        match val[0..3].parse::<u64>() {
                            Ok(height) => {
                                if height < 150 || height > 193 {
                                    return false;
                                }
                            }
                            _ => return false,
                        }
                    } else if val.contains("in") {
                        if val.len() != 4 {
                            return false;
                        }
                        match val[0..2].parse::<u64>() {
                            Ok(height) => {
                                if height < 59 || height > 76 {
                                    return false;
                                }
                            }
                            _ => return false,
                        }
                    } else {
                        return false;
                    }
                }
                "hcl" => {
                    let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
                    if !re.is_match(val) {
                        return false;
                    }
                }
                "ecl" => {
                    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val) {
                        return false;
                    }
                }
                "pid" => {
                    let re = Regex::new("^[0-9a-f]{9}$").unwrap();
                    if !re.is_match(val) {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    fields.is_empty()
}

fn main() {
    let input = stdin();
    let mut current = String::new();
    let mut valid = 0;
    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => {
                if process_entry(&current) {
                    valid += 1;
                }
                break;
            }
            Ok(_) => {}
            _ => panic!("Failed to read stdin"),
        }
        let buf = buf.trim();
        if buf.is_empty() {
            if process_entry(&current) {
                valid += 1;
            }
            current = String::new();
        } else {
            if !current.is_empty() {
                current.push_str(" ");
            }
            current.push_str(&buf);
        }
    }
    println!("{}", valid);
}
