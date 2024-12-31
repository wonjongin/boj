use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug)]
struct CharCount {
    char: char,
    count: i32,
}

fn main() {
    let stdin = io::stdin();
    // let mut now = Instant::now();
    let mut input_iter = stdin.lock().lines();
    let str: String = input_iter.next().unwrap().unwrap();
    let chars: Vec<char> = str.chars().collect();
    let mut btr: HashMap<char, i32> = HashMap::new();
    chars.iter().for_each(|&x| *btr.entry(x).or_insert(0) += 1);

    let mut ctr: Vec<CharCount> = btr
        .iter()
        .map(|(&k, &v)| CharCount { char: k, count: v })
        .collect();

    ctr.sort_by(|a, b| b.count.cmp(&a.count));
    let sum = ctr.iter().fold(0, |acc, x| acc + x.count);
    let maximun = ctr[0].count;
    if maximun > sum - maximun + 1 {
        println!("{}", 0)
    } else {
        // 경우의 수 구하기
    }
    // https://www.acmicpc.net/problem/1342
    println!("{:?}", ctr);
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
