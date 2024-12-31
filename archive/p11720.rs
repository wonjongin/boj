use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    let mut input_iter = stdin.lock().lines();
    let n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();
    let ll: String = input_iter.next().unwrap().unwrap();
    let res: u32 = ll.chars().map(|x| x.to_digit(10).unwrap()).sum();

    // Something ...

    println!("{}", res)
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
