use std::io::{self};

fn is_prime(num: u32) -> bool {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let mut res = true;
    if primes.contains(&num) {
        return res;
    }
    if num == 1 {
        return false;
    }
    primes.iter().for_each(|x| {
        if num % x == 0 {
            res = false;
        }
    });
    res
}

fn main() {
    let mut input = String::new();
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<&str> = input.split_whitespace().collect();
    let m: Vec<u32> = n.iter().map(|x| x.parse().unwrap()).collect();

    let mut res = 0;

    m.iter().for_each(|&x| {
        if is_prime(x) {
            res += 1;
        }
    });

    println!("{}", res);
}
