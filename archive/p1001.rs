use std::io::{self, BufRead};

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let stdin = io::stdin();
    let line1: String = stdin.lock().lines().next().unwrap().unwrap();
    let line2: Vec<&str> = line1.split(" ").collect();

    println!(
        "{}",
        sub(line2[0].parse().unwrap(), line2[1].parse().unwrap())
    );
}
