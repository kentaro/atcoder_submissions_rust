use std::cmp::*;
use proconio::input;

fn main() {
    input! {
      n: usize,
      h: [i32; n],
    }

    solve(n, h);
}

fn solve(n: usize, h: Vec<i32>) {
    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();

    for i in 2..n {
        dp[i] = min(
            dp[i - 1] + (h[i - 1] - h[i]).abs(),
            dp[i - 2] + (h[i - 2] - h[i]).abs(),
        );
    }

    println!("{}", dp[n - 1]);
}
