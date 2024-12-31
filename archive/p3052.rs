use std::io::{self};

fn main() {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<i32> = Vec::new();
    for _ in 0..10 {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        inputs.push(line.trim().parse().unwrap());
    }

    let mut rs: Vec<i32> = inputs.iter().map(|x| x % 42).collect();
    rs.sort();
    rs.dedup();

    println!("{}", rs.len());
}
