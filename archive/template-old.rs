use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    let mut input_iter = stdin.lock().lines();
    let n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();
    let mut inputs: Vec<String> = Vec::new();
    for i in 0..n {
        let ll: String = input_iter.next().unwrap().unwrap();
        if ll == "" {
            break;
        }
        inputs.push(ll);
    }

    // Something ...

    println!("Answer")
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
