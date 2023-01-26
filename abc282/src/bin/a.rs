use proconio::*;

fn main() {
  input! {
    k: u8,
  }

  solve(k);
}

fn solve(k: u8) {
  for i in 0..k {
    let c = (i + 65) as char;
    print!("{}", c);
  }

  println!("");
}
