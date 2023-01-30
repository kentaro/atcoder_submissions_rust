use proconio::*;

fn main() {
    input! { s: String }
    solve(s);
}

fn solve(s: String) {
  let mut ans = 0;

  for (i, c) in s.chars().enumerate() {
    if c == 'a' {
      ans = i + 1;
    }
  }

  if ans > 0 {
    println!("{}", ans);
  } else {
    println!("-1");
  }
}
