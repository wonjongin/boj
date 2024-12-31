use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    let mut input_iter = stdin.lock().lines();
    // let n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();
    let ll: String = input_iter.next().unwrap().unwrap();

    for i in 97..123 {
        let character = std::char::from_u32(i).unwrap();
        let mut res: i32 = -1;
        for (p, c) in ll.chars().enumerate() {
            if c == character {
                res = p as i32;
                break;
            }
        }
        print!("{} ", res)
    }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
