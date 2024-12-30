use std::io::{self, BufRead};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let stdin = io::stdin();
    let line1: String = stdin.lock().lines().next().unwrap().unwrap();
    let line2: Vec<&str> = line1.split(" ").collect();

    println!(
        "{}",
        add(line2[0].parse().unwrap(), line2[1].parse().unwrap())
    );
}
