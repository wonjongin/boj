use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for i in 0..n {
        let arr = vec!["*"; i + 1];
        println!("{}", arr.join(""))
    }
}
