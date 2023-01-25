use itertools::Itertools;
use proconio::*;

fn main() {
  input! {
    n: usize,
    p: usize,
    q: usize,
    r: usize,
    s: usize,
    a: [u8; n],
  }

  solve(n, p, q, r, s, a);
}

fn solve(n: usize, p: usize, q: usize, r: usize, s: usize, a: Vec<u8>) {
  let mut ans: Vec<u8> = vec![];

  (0..p-1).for_each(|i| ans.push(a[i]));
  (r-1..s).for_each(|i| ans.push(a[i]));
  (q..r-1).for_each(|i| ans.push(a[i]));
  (p-1..q).for_each(|i| ans.push(a[i]));
  (s..n).for_each(|i| ans.push(a[i]));

  let str = ans.iter().map(|&x| x.to_string()).join(" ");
  println!("{}", str);
}
