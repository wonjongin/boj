use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();
    let words: Vec<&str> = line.split(" ").collect();
    let res = words.into_iter().filter(|&x| x != "").count();

    println!("{}", res);
}
