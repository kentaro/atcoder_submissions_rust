use proconio::input;

fn main() {
    input! { s: String }
    solve(s);
}

fn solve(s: String) {
    let mut ans = "".to_string();

    for c in s.chars() {
        match c {
            '0' => ans.push(char::from('1')),
            '1' => ans.push(char::from('0')),
            _ => {}
        }
    }

    println!("{}", ans);
}
