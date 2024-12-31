use std::{
    collections::VecDeque,
    io::{self},
};

// 문서가 프린트 되면 true, 뒤로 넘어가면 false
fn printer(queue: &mut VecDeque<i32>) -> bool {
    if queue.iter().max() == queue.front() {
        queue.pop_front();
        true
    } else {
        if let Some(temp) = queue.pop_front() {
            queue.push_back(temp);
        }
        false
    }
}

// 원하는 문서가 출력되면 1 반환 아니면 0, 원하지 않지만 출력되면 -1
fn move_doc(queue: &mut VecDeque<i32>, doc_cur: &mut u32) -> i32 {
    if *doc_cur == 0 {
        if printer(queue) {
            1
        } else {
            *doc_cur = queue.len() as u32 - 1;
            0
        }
    } else {
        *doc_cur -= 1;
        match printer(queue) {
            true => -1,
            false => 0,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<Vec<String>> = Vec::new();
    for _ in 0..n {
        let mut line1 = String::new();
        let mut line2 = String::new();
        if io::stdin().read_line(&mut line1).unwrap() == 0 {
            break;
        }
        if io::stdin().read_line(&mut line2).unwrap() == 0 {
            break;
        }
        if line1.trim().is_empty() {
            break;
        }
        if line2.trim().is_empty() {
            break;
        }
        inputs.push(vec![line1.trim().to_string(), line2.trim().to_string()]);
    }

    let res: Vec<i32> = inputs
        .iter()
        .map(|v| {
            let mut index = 1;
            let vi: Vec<&str> = v[0].split_whitespace().collect();
            let (doc_count, doc_index): (u32, u32) =
                (vi[0].parse().unwrap(), vi[1].parse().unwrap());
            let mut doc_cur = doc_index;
            let vf: Vec<&str> = v[1].split_whitespace().collect();
            let mut queue: VecDeque<i32> = vf.iter().map(|x| x.parse().unwrap()).collect();
            loop {
                match move_doc(&mut queue, &mut doc_cur) {
                    1 => break,
                    0 => (),
                    -1 => index += 1,
                    _ => (),
                }
                // println!("i:{} cur:{} {:?}", index, doc_cur, queue);
            }
            index
        })
        .collect();

    res.iter().for_each(|x| println!("{}", x));
}
