use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();
    let mut control = String::new();
    let mut first = false;
    let mut second = false;
    let mut res = String::new();
    let mut searches: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        if !first {
            first = true;
            continue;
        }
        let ll = line.unwrap();
        if ll == "" {
            break;
        }
        if !second {
            second = true;
            control = ll.clone();
        }
        searches.push(ll);
    }

    for (i, c) in control.chars().enumerate() {
        let eqs = searches.iter().all(|x| x.chars().nth(i).unwrap() == c);
        let r = if eqs { c } else { '?' };
        res.push(r);
    }

    println!("{}", res);
}
