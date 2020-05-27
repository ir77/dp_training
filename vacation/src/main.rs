#[allow(unused_imports)]
use std::cmp::max;
use std::io;

// - ドキュメントの提示
// - 補完が効くこと
// - シンボルを理解してrename
// - 定義元ジャンプ⇄参照先ジャンプ
// - 警告、エラーの表示
// - フォーマット

fn main() {
    let input = input_number();
    let n = input[0] as usize;
    let mut ai: Vec<i64> = Vec::new();
    let mut bi: Vec<i64> = Vec::new();
    let mut ci: Vec<i64> = Vec::new();

    for _ in 0..n {
        let input = input_number();
        ai.push(input[0]);
        bi.push(input[1]);
        ci.push(input[2]);
    }

    let mut dpa = vec![i64::max_value(); n];
    let mut dpb = vec![i64::max_value(); n];
    let mut dpc = vec![i64::max_value(); n];

    for i in 0..n {
        if i == 0 {
            dpa[i] = ai[i];
            dpb[i] = bi[i];
            dpc[i] = ci[i];
            continue;
        }

        dpa[i] = ai[i] + max(dpb[i - 1], dpc[i - 1]);
        dpb[i] = bi[i] + max(dpa[i - 1], dpc[i - 1]);
        dpc[i] = ci[i] + max(dpb[i - 1], dpa[i - 1]);
    }

    println!("{}", max(max(dpa[n - 1], dpb[n - 1]), dpc[n - 1]));
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
