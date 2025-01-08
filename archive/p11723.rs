use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut s: Vec<bool> = vec![false; 20];
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }

        let ll: Vec<&str> = line.split_whitespace().collect();
        let command: &str = ll[0];
        let arg: usize = if ll.len() > 1 {
            ll[1].parse().unwrap_or(0) - 1
        } else {
            0
        };

        if command == "add" {
            s[arg] = true;
        } else if command == "remove" {
            s[arg] = false;
        } else if command == "check" {
            if s[arg] {
                writeln!(out, "1").unwrap();
            } else {
                writeln!(out, "0").unwrap();
            }
        } else if command == "toggle" {
            s[arg] = !s[arg]
        } else if command == "all" {
            s = vec![true; 20];
        } else if command == "empty" {
            s = vec![false; 20];
        } else {
            std::process::exit(0)
        }
    }
}
