use std::io::{self};

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let str = input.trim();
        if str == "0" {
            break;
        }
        if str.chars().rev().collect::<String>() == str {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
