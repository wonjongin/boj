use std::io::{self};

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

    let mut stack: Vec<u32> = Vec::new();
    inputs.iter().for_each(|x| {
        if x.starts_with("push") {
            let args: Vec<&str> = x.split_whitespace().collect();
            stack.push(args[1].parse().unwrap());
        } else if x.starts_with("pop") {
            let out: i32 = match stack.pop() {
                Some(value) => value as i32,
                None => -1,
            };
            println!("{}", out);
        } else if x.starts_with("size") {
            println!("{}", stack.len());
        } else if x.starts_with("empty") {
            let is_empty = if stack.is_empty() { 1 } else { 0 };
            println!("{}", is_empty);
        } else if x.starts_with("top") {
            let out: i32 = match stack.last() {
                Some(value) => *value as i32,
                None => -1,
            };
            println!("{}", out);
        } else {
            println!("Command Not Found");
        }
    });
}
