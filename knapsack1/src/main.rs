#[allow(unused_imports)]
use std::cmp::max;
use std::io;

/*
3 8
3 30
4 50
5 60
*/

/*
5 5
1 1000000000
1 1000000000
1 1000000000
1 1000000000
1 1000000000
*/

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let w = input[1] as usize;
    let input = (0..n).map(|_| input_number()).collect::<Vec<Vec<i64>>>();
    let ws: Vec<i64> = input.iter().map(|x| x[0]).collect();
    let vs: Vec<i64> = input.iter().map(|x| x[1]).collect();

    let mut dp = vec![vec![0; 100100]; 110];

    for i in 0..n {
        for j in 0..w + 1 {
            if j >= ws[i] as usize {
                let on_the_left = dp[i][j - ws[i] as usize] + vs[i];
                let target = &mut dp[i + 1][j];
                update(target, on_the_left);
            }
            let upper = dp[i][j];
            let target = &mut dp[i + 1][j];
            update(target, upper);
        }
    }

    // for i in 0..n + 1 {
    //     for j in 0..w + 1 {
    //         print!("{}, ", dp[i][j]);
    //     }
    //     println!("");
    // }

    println!("{}", dp[n][w]);
}

#[allow(dead_code)]
fn update<T>(left: &mut T, right: T)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    if *left < right {
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
