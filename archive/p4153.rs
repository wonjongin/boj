use std::io::{self};

fn main() {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    //let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim() == "0 0 0" {
            break;
        }
        inputs.push(line.trim().to_string());
    }

    let mut res: Vec<String> = Vec::new();

    res = inputs
        .iter()
        .map(|x| {
            let mut nums: Vec<i32> = x.split_whitespace().map(|x| x.parse().unwrap()).collect();
            nums.sort();
            if nums[2].pow(2) == nums[0].pow(2) + nums[1].pow(2) {
                return "right".to_string();
            }
            "wrong".to_string()
        })
        .collect();

    res.iter().for_each(|x| {
        println!("{}", x);
    });
}
