use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<&str> = input.split_whitespace().collect();
    let m: Vec<u32> = n.iter().map(|x| x.parse().unwrap()).collect();

    let a = m[0];
    let b = m[1];
    let v = m[2];

    let avg = a - b;

    // let remain = v % avg;
    // let mul = v - remain;

    let res = ((v as f64 - a as f64) / avg as f64).ceil() as u32 + 1;

    println!("{}", res);
}
