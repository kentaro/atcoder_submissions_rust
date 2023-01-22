use proconio::input;

fn main() {
    input! {
      n: usize,
      s: [String; n],
    }

    solve(s);
}

fn solve(s: Vec<String>) {
  for i in s.into_iter().rev() {
    println!("{}", i);
  }
}
