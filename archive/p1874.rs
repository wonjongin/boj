use std::{
    collections::VecDeque,
    io::{self, Write},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut inputs: Vec<u32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        inputs.push(line.trim().parse().unwrap());
    }

    let want: Vec<u32> = inputs.clone();
    let mut res: Vec<&str> = vec!["+"];
    let mut queue: VecDeque<u32> = (2..=n).collect::<Vec<u32>>().into();
    let mut stack: Vec<u32> = vec![1];
    let mut cur: usize = 0;

    loop {
        if cur as u32 == n {
            break;
        }

        if queue.is_empty() && stack.is_empty() {
            break;
        }

        if queue.is_empty() {
            let temp = want[want.len() - stack.len()..].to_vec();
            let mut temp2 = stack.clone();
            temp2.sort_by(|a, b| b.cmp(a));
            if temp != temp2 {
                println!("NO");
                std::process::exit(0);
            }
        }

        if stack.is_empty() {
            stack.push(queue.pop_front().unwrap());
            res.push("+");
            continue;
        }

        if want[cur] == *stack.last().unwrap() {
            cur += 1;
            stack.pop().unwrap();
            res.push("-")
        } else {
            stack.push(queue.pop_front().unwrap());
            res.push("+");
        }

        // println!("+--cur: {}", cur);
        // println!("|queue: {:?}", queue);
        // println!("| want: {:?}", want);
        // println!("|stack: {:?}", stack);
        // println!("+--res: {:?}", res);
        // println!();
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    res.iter().for_each(|x| writeln!(out, "{}", x).unwrap());
}
