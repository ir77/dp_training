#[allow(unused_imports)]
use std::cmp::max;
use std::io;

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let w = input[1] as usize;
    let input = (0..n).map(|_| input_number()).collect::<Vec<Vec<i64>>>();
    let ws: Vec<i64> = input.iter().map(|x| x[0]).collect();
    let vs: Vec<i64> = input.iter().map(|x| x[1]).collect();

    let max_v = 100100usize;

    let mut dp = vec![vec![1000000100; max_v]; 110];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..max_v {
            if j >= vs[i] as usize {
                let on_the_left = dp[i][j - vs[i] as usize] + ws[i];
                let target = &mut dp[i + 1][j];
                update_min(target, on_the_left);
            }
            let upper = dp[i][j];
            let target = &mut dp[i + 1][j];
            update_min(target, upper);
        }
    }

    let mut res = 0usize;
    for j in 0..max_v {
        if dp[n][j] <= w as i64 {
            res = j
        }
    }
    println!("{}", res);
}

#[allow(dead_code)]
fn update_max<T>(left: &mut T, right: T)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    if *left < right {
        *left = right.clone();
    }
}

#[allow(dead_code)]
fn update_min<T>(left: &mut T, right: T)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    if *left > right {
        *left = right.clone();
    }
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
