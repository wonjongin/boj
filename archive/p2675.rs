use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    let mut input_iter = stdin.lock().lines();
    let n: usize = input_iter.next().unwrap().unwrap().parse().unwrap();

    for i in 0..n {
        let ll: String = input_iter.next().unwrap().unwrap();
        if ll == "" {
            break;
        }
        let ss: Vec<&str> = ll.split(" ").collect();
        let rr: usize = ss[0].parse().unwrap();
        let str: &str = ss[1];
        str.chars().for_each(|x| {
            for r in 0..rr {
                print!("{}", x);
            }
        });
        print!("\n");
    }

    // Something ...

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
