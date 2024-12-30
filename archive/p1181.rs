use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();

    let mut first = false;
    let mut n = 0;
    let mut words: BTreeSet<String> = BTreeSet::new();
    let mut lens: BTreeSet<usize> = BTreeSet::new();

    for line in stdin.lock().lines() {
        let ll: String = line.unwrap();
        if !first {
            first = true;
            n = ll.parse().unwrap();
            continue;
        }
        if ll == "" {
            break;
        }

        words.insert(ll.clone());
        lens.insert(ll.len());
    }

    // words.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));
    for i in lens {
        let temp: Vec<&str> = words
            .iter()
            .map(|s| s.as_str())
            .filter(|x| x.len() == i)
            .collect();
        println!("{}", temp.join("\n"));
    }
}
