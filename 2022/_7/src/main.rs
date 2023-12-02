use std::io::stdin;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Dir {
    name: Vec<String>,
    dirs: HashSet<String>,
    files: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
enum Line {
    Cd(String),
    Dir(String),
    File(usize, String),
}

fn main() {
    let mut fs: HashMap<String, Dir> = HashMap::new();
    let root = Dir {
        name: Vec::new(),
        dirs: HashSet::new(),
        files: HashMap::new(),
    };
    fs.insert("".to_string(), root);
    let mut dir = fs.get_mut(&"".to_string()).unwrap();

    let strings: Vec<_> = stdin().lines().map(|line| line.unwrap()).collect();
    let mut lines = Vec::new();
    for line in strings {
        if line.starts_with("$ cd") {
            lines.push(Line::Cd(line[5..].to_string()));
        } else if line.starts_with("dir ") {
            lines.push(Line::Dir(line[4..].to_string()));
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            let elems: Vec<_> = line.split(' ').collect();
            lines.push(Line::File(elems[0].parse().unwrap(), elems[1].to_string()));
        }
    }
    for line in lines {
        match line {
            Line::Cd(name) => {
                // Command
                let mut new_name = dir.name.clone();
                if &name == ".." {
                    if !new_name.is_empty() {
                        new_name.pop();
                        let key = new_name.join("/");
                        dir = fs.get_mut(&key).unwrap();
                    }
                } else if &name == "/" {
                    dir = fs.get_mut(&"".to_string()).unwrap();
                } else {
                    new_name.push(name);
                    let key = new_name.join("/");
                    dir.dirs.insert(key.clone());
                    dir = fs.entry(key.clone()).or_insert_with(|| Dir {
                        name: new_name,
                        dirs: HashSet::new(),
                        files: HashMap::new(),
                    });
                }
                continue;
            },
            Line::Dir(name) => {
                let old_name = dir.name.clone();
                let mut new_name = dir.name.clone();
                new_name.push(name);
                let key = new_name.join("/");
                dir.dirs.insert(key.clone());
                dir = fs.entry(key.clone()).or_insert_with(|| Dir {
                    name: new_name,
                    dirs: HashSet::new(),
                    files: HashMap::new(),
                });
                dir = fs.get_mut(&old_name.join("/")).unwrap();
            },
            Line::File(size, name) => {
                dir.files.insert(name, size);
            }
        }
    }
    let total = walk("".to_string(), &fs).1;
    let target = 30_000_000 - (70_000_000 - total);
    println!("{}", walk2("".to_string(), &fs, target).0)
}

fn walk(path: String, fs: &HashMap<String, Dir>) -> (usize, usize) {
    let dir = fs.get(&path).unwrap();
    let mut tsize = 0;
    let mut size = 0;
    for file_size in dir.files.values() {
        size += file_size;
    }
    for child in &dir.dirs {
        let (ctsize, csize) = walk(child.clone(), fs);
        tsize += ctsize;
        size += csize;
    }
    if size <= 100_000 {
        tsize += size;
    }
    (tsize, size)
}

fn walk2(path: String, fs: &HashMap<String, Dir>, target: usize) -> (usize, usize) {
    let dir = fs.get(&path).unwrap();
    let mut size = 0;
    let mut ans = 999999999999999999;
    for file_size in dir.files.values() {
        size += file_size;
    }
    for child in &dir.dirs {
        let (candidate, csize) = walk2(child.clone(), fs, target);
        size += csize;
        ans = ans.min(candidate);
    }
    if size >= target {
        ans = ans.min(size);
    }
    (ans, size)
}
