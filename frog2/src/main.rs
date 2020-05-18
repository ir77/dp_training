#[allow(unused_imports)]
use std::cmp;
use std::io;

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let k = input[1] as usize;
    let his = input_number();
    let mut dp = vec![i64::max_value(); n];

    dp[0] = 0;
    for i in 0..n {
        for j in 1..k + 1 {
            if i + j >= n {
                continue;
            }

            dp[i + j] = cmp::min(dp[i + j], dp[i] + (his[i] - his[i + j]).abs());
        }
    }

    println!("{}", dp.last().unwrap());
}

#[allow(dead_code)]
fn input_number() -> Vec<i64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let vec: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    vec
}

#[allow(dead_code)]
fn input() -> (char, usize) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    let mut iter = s.trim().split_whitespace();
    let c: char = iter.next().unwrap().chars().nth(0).unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    (c, n)
}
