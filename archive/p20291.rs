use std::collections::BTreeMap;
use std::convert::TryInto;
use std::io::{self, BufRead};

struct Filename {
    name: String,
    ext: String,
}

fn main() {
    let stdin = io::stdin();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();

    let mut first = false;
    let mut files: Vec<Filename> = Vec::new();

    for line in stdin.lock().lines() {
        if !first {
            first = true;
            continue;
        }
        let ll = line.unwrap();
        if ll == "" {
            break;
        }

        let [name, ext]: [String; 2] = ll
            .trim()
            .split(".")
            .map(String::from)
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        files.push(Filename { name, ext })
    }

    let mut exts: BTreeMap<String, i32> = BTreeMap::new();

    files
        .iter()
        .for_each(|x| *exts.entry(x.ext.clone()).or_insert(0) += 1);
    exts.iter().for_each(|(x, y)| println!("{} {}", x, y));
}
