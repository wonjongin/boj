use std::convert::TryInto;
use std::io::{self, BufRead};

fn grade_to_f(grade: &str) -> Result<f64, String> {
    let res: f64 = match grade {
        "A+" => 4.5,
        "A0" => 4.0,
        "B+" => 3.5,
        "B0" => 3.0,
        "C+" => 2.5,
        "C0" => 2.0,
        "D+" => 1.5,
        "D0" => 1.0,
        "F" => 0.0,
        _ => return Err(String::from(format!("Grade {} not invalid", grade))),
    };
    Ok(res)
}

fn main() {
    let stdin = io::stdin();
    // let line: String = stdin.lock().lines().next().unwrap().unwrap();
    let mut sum = 0.0;
    let mut units = 0.0;
    let mut index = 0;
    for line in stdin.lock().lines() {
        if index >= 20 {
            break;
        }
        index += 1;
        let ll = line.unwrap();
        if ll == "" {
            break;
        }
        let [_, unit, grade]: [&str; 3] = ll
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        if grade == "P" {
            continue;
        }
        sum += grade_to_f(&grade).unwrap() * unit.parse::<f64>().unwrap();
        units += unit.parse::<f64>().unwrap();
    }

    println!("{}", sum / units)
}
