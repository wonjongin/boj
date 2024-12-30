use std::io::{self, BufRead};

fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
    
}

fn main() {
    let stdin = io::stdin();
    let line1: String = stdin.lock().lines().next().unwrap().unwrap();
    let line2: Vec<&str> = line1.split(" ").collect();

    println!(
        "{}",
        div(line2[0].parse().unwrap(), line2[1].parse().unwrap()).unwrap()
    );
}
