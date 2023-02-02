use im_rc::HashMap;
use proconio::input;

fn main() {
    input! { s: String }
    solve(s);
}

fn solve(s: String) {
    let mut ans = 0;
    let mut map: HashMap<char, u8> = HashMap::new();
    map.insert('v', 1);
    map.insert('w', 2);

    for c in s.chars() {
        ans += map.get(&c).unwrap();
    }

    println!("{}", ans);
}
