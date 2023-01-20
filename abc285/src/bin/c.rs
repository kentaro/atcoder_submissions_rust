use num_traits::Pow;

fn main() {
  proconio::input! {
    s: String,
  }

  solve(s);
}

fn solve(s: String) {
  let mut total: u64 = 0;

  for (i, c) in s.chars().rev().enumerate() {
    let code = c as u64;
    total += (code - 64) * Pow::pow(26_u64, i);
  }

  println!("{}", total);
}
