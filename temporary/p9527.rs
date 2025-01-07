use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<&str> = input.split_whitespace().collect();
    let m: Vec<u64> = n.iter().map(|x| x.parse().unwrap()).collect();

    let mut res = 0;

    for x in m[0]..(m[1] + 1) {
        for i in 0..64 {
            if ((x >> i) & 1) == 1 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
