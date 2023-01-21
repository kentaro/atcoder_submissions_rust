use proconio::marker::Chars;

fn main() {
  proconio::input! {
    n: usize,
    s: Chars,
  }

  solve(n, s);
}

fn solve(n: usize, s: Vec<char>) {
  for i in 1..n {
    let mut l = 0;

    for j in 0..(n - i) {
      if s[j] != s[j + i] {
        l += 1;
      } else {
        break;
      }
    }

    println!("{}", l);
  }
}
