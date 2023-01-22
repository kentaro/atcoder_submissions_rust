use proconio::input;
use num_traits::Pow;

fn main() {
    input! {
      a: u32,
      b: u32,
    }

    solve(a, b);
}

fn solve(a: u32, b: u32) {
  println!("{}", Pow::pow(a, b));
}
