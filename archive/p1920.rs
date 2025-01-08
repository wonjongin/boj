use std::{
    collections::HashSet,
    io::{self, Write},
};

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let n1: usize = input1.trim().parse().unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let a: HashSet<isize> = input2
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).unwrap();
    let n2: usize = input3.trim().parse().unwrap();

    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).unwrap();
    let m: Vec<isize> = input4
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let res: Vec<usize> = m
        .iter()
        .map(|x| match a.contains(x) {
            true => 1,
            false => 0,
        })
        .collect();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    res.iter().for_each(|x| writeln!(out, "{}", x).unwrap());
}
