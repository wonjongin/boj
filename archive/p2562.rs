use std::io::{self};

fn main() {
    let n: i32 = 9;

    let mut inputs = Vec::with_capacity(9);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let num: usize = line.trim().parse().unwrap();
        inputs.push(num);
    }

    let mut max = 0;
    let mut index = 0;

    for (i, &v) in inputs.iter().enumerate() {
        if max < v {
            max = v;
            index = i;
        }
    }

    println!("{}", max);
    println!("{}", index + 1);
}
