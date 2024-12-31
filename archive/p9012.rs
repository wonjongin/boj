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

    let res = inputs.iter().map(|s| {
        let mut open = 0;
        let mut close = 0;
        let mut cur = 0;
        let mut no = false;
        s.chars().for_each(|c| {
            if c == '(' {
                open += 1;
                cur += 1;
            }
            if c == ')' {
                close += 1;
                cur -= 1;
            }
            if cur < 0 {
                no = true;
                return;
            }
        });
        if no {
            "NO"
        } else if open == close {
            "YES"
        } else {
            "NO"
        }
    });

    res.for_each(|x| println!("{}", x));
}
