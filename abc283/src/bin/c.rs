use proconio::input;

fn main() {
  input! {
    s: String,
  }

  solve(s);
}

fn solve(s: String) {
  let mut count = 0;
  let mut skip = false;
  let len = s.len();

  for (i, c) in s.chars().enumerate() {
    if c == '0' && !skip && i < len - 1 {
      count += 1;
      skip = true;
    } else if c == '0' && skip {
      skip = false;
    } else {
      count += 1;
      skip = false;
    }
  }

  println!("{}", count);
}
