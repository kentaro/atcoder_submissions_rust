use proconio::input;

fn main() {
  input! {
    t: usize,
  }

  solve(t);
}

fn solve(t: usize) {
  for _ in 0..t {
    input!{
      n: usize,
      a: [i32; n],
    }

    let count = a.iter().filter(|&&x| x % 2 != 0).count();
    println!("{}", count);
  }
}
