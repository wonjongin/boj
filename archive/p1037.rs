use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();

    let mut first = false;
    let mut n = 0;
    let mut nums: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        let ll: String = line.unwrap();
        if !first {
            first = true;
            n = ll.parse().unwrap();
            continue;
        }
        if ll == "" {
            break;
        }

        nums = ll.split(" ").map(|x| x.parse().unwrap()).collect();
        break;
    }
    nums.sort();
    let res: i32 = if n % 2 == 1 {
        nums.get(nums.len() / 2).unwrap().pow(2)
    } else {
        nums.first().unwrap() * nums.last().unwrap()
    };

    println!("{}", res);
}
