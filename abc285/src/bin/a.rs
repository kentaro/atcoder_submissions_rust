fn main() {
  proconio::input! {
    a: i32,
    b: i32,
  }

  if 2 * a == b || (2 * a + 1) == b {
    println!("Yes")
  } else {
    println!("No")
  }
}
