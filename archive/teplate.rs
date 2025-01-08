use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<String> = Vec::with_capacity(n);
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

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    inputs.iter().for_each(|x| writeln!(out, "{}", x).unwrap());
}
