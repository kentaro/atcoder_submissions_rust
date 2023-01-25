use proconio::*;

fn main() {
  input! {
    n: usize,
    s: String,
  }

  solve(n, s);
}

fn solve(_n: usize, s: String) {
  let ans = s.replace("na", "nya");
  println!("{}", ans);
}
