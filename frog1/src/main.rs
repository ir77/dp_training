use std::cmp;
use std::io;

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let his = input_number();
    let mut dp = vec![i64::max_value(); n];

    for (index, _) in his.iter().enumerate() {
        if index == 0 {
            dp[0] = 0;
            continue;
        }
        dp[index] = dp[index - 1] + (his[index] - his[index - 1]).abs();
        if index > 1 {
            dp[index] = cmp::min(
                dp[index],
                dp[index - 2] + (his[index] - his[index - 2]).abs(),
            );
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
