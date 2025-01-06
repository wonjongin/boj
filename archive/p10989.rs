use std::io::{self, BufWriter, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut count = vec![0; 10001]; // 1~10000까지의 수를 직접 인덱스로 사용

    for _ in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num: usize = num.trim().parse().unwrap();
        count[num] += 1; // 인덱스를 num 그대로 사용
    }

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 1..=10000 {
        // 1부터 10000까지
        for _ in 0..count[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
    writer.flush().unwrap();
}
