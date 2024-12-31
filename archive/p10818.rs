use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines();
    let n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();

    let ll: String = input_iter.next().unwrap().unwrap();
    let nums: Vec<u32> = ll.split(" ").map(|x| x.parse().unwrap()).collect();
    let mut minv = 1_000_000;
    let mut maxv = 0;

    nums.iter().for_each(|&x| {
        minv = if minv > x { x } else { minv };
        maxv = if maxv < x { x } else { maxv };
    });
    println!("{} {}", minv, maxv);
}
