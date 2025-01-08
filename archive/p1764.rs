use std::{
    collections::{BTreeSet, HashSet},
    io::{self, Write},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut d: HashSet<String> = HashSet::with_capacity(n[0]);
    let mut b: HashSet<String> = HashSet::with_capacity(n[1]);
    for _ in 0..n[0] {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        d.insert(line.trim().to_string());
    }

    for _ in 0..n[1] {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        b.insert(line.trim().to_string());
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut res: BTreeSet<String> = BTreeSet::new();
    d.iter().for_each(|x| {
        if b.contains(x) {
            res.insert(x.to_string());
        }
    });
    writeln!(out, "{}", res.len()).unwrap();
    res.iter().for_each(|x| writeln!(out, "{}", x).unwrap())
}
