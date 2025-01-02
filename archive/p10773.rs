use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        inputs.push(line.trim().to_string());
    }

    let mut temp: Vec<u64> = Vec::new();
    inputs.iter().for_each(|x| {
        let num: u64 = x.parse().unwrap();
        if num == 0 {
            temp.pop();
        } else {
            temp.push(num);
        }
    });

    println!("{}", temp.iter().sum::<u64>());
}
