use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [u32; n],
      q: usize,
    }

    solve(a, q);
}

fn solve(mut a: Vec<u32>, q: usize) {
  for _ in 0..q {
    input! {
      l: usize,
    }

    match l {
      1 => {
        input! {
          query: [u32; 2],
        }

        a[(query[0] as usize) - 1] = query[1];
      }
      2 => {
        input! {
          query: [u32; 1],
        }

        println!("{}", a[(query[0] as usize) - 1]);
      }
      _ => {}
    }
  }
}
