use std::{
    collections::VecDeque,
    io::{self, Write},
};

struct Case {
    length: usize,
    command: String,
    arr: Vec<u32>,
}

impl Case {
    fn run(&self) -> String {
        let mut res: Vec<u32> = self.arr.clone();

        for c in self.command.chars() {
            match c {
                'R' => res.reverse(),
                'D' => {
                    if res.is_empty() {
                        return "error".to_string();
                    } else {
                        res.remove(0);
                    }
                }
                _ => return "error".to_string(),
            }
        }
        let astr = res
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        format!("[{}]", astr)
    }

    fn fast(&self, out: &mut io::BufWriter<impl Write>) {
        let mut front = true;
        let mut res: VecDeque<u32> = self.arr.clone().into();

        for c in self.command.chars() {
            match c {
                'R' => front = !front,
                'D' => {
                    if res.is_empty() {
                        writeln!(out, "error").unwrap();
                        return;
                    }
                    if front {
                        res.pop_front();
                    } else {
                        res.pop_back();
                    }
                }
                _ => {
                    writeln!(out, "error").unwrap();
                    return;
                }
            }
        }

        if res.is_empty() {
            writeln!(out, "[]").unwrap();
            return;
        }

        write!(out, "[").unwrap();

        if front {
            for (i, x) in res.iter().enumerate() {
                if i == res.len() - 1 {
                    write!(out, "{}", x).unwrap();
                } else {
                    write!(out, "{},", x).unwrap();
                }
            }
        } else {
            for i in (1..res.len()).rev() {
                write!(out, "{},", res[i]).unwrap();
            }
            write!(out, "{}", res[0]).unwrap();
        }

        writeln!(out, "]").unwrap();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut inputs: Vec<Case> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line: Vec<String> = vec![String::new(); 3];

        for i in 0..3 {
            if io::stdin().read_line(&mut line[i]).unwrap() == 0 {
                break;
            }
        }

        if line[0].trim().is_empty() {
            break;
        }
        // println!("{:?}", line);
        inputs.push(Case {
            command: line[0].trim().to_string(),
            length: line[1].trim().parse().unwrap(),
            arr: if line[2].trim().eq("[]") {
                vec![]
            } else {
                line[2]
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect()
            },
        });
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    inputs.iter().for_each(|x| x.fast(&mut out));
}
