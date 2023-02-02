use proconio::input;

fn main() {
    input! {
      s: String,
      t: String,
    }
    solve(s, t);
}

fn solve(s: String, t: String) {
    let matches: Vec<&str> = s.matches(&t).collect();

    if matches.len() > 0 {
      println!("Yes")
    } else {
      println!("No")
    }
}
