use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();
    // let mut input_iter = stdin.lock().lines();
    let mut first = false;
    // let mut n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();
    let mut arr: Vec<usize> = vec![0, 1];
    let mut n = 0;
    for i in 0..40 {
        arr.push(arr[i] + arr[i + 1])
    }
    let mut inputs: Vec<usize> = Vec::new();

    for line in stdin.lock().lines() {
        let ll: String = line.unwrap();
        if !first {
            first = true;
            n = ll.parse().unwrap();
            continue;
        }
        if ll == "" {
            // now = Instant::now();
            break;
        }
        inputs.push(ll.parse().unwrap())
    }

    inputs.iter().for_each(|&t| {
        if t == 1 {
            println!("{} {}", 0, 1);
        } else if t == 0 {
            println!("{} {}", 1, 0);
        } else {
            println!("{} {}", arr[t - 1], arr[t]);
        }
    });
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
