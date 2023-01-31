use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        w: i32,
        blocks: [Chars; h],
    }

    solve(h, w, blocks);
}

fn solve(h: i32, w: i32, blocks: Vec<Vec<char>>) {
    let mut x = 0;
    let mut y = 0;
    let mut seen: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];

    for i in 0..h {
        for j in 0..w {
            if blocks[i as usize][j as usize] == 's' {
                x = i;
                y = j;
            }
        }
    }

    if search(x, y, h, w, &blocks, &mut seen) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn search(
    x: i32,
    y: i32,
    h: i32,
    w: i32,
    blocks: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
) -> bool {
    if x < 0 || x >= h || y < 0 || y >= w {
        return false;
    }
    if seen[x as usize][y as usize] {
        return false;
    }
    if blocks[x as usize][y as usize] == '#' {
        return false;
    }

    seen[x as usize][y as usize] = true;

    if blocks[x as usize][y as usize] == 'g' {
        return true;
    }

    return search(x + 1, y, h, w, blocks, seen)
        || search(x - 1, y, h, w, blocks, seen)
        || search(x, y + 1, h, w, blocks, seen)
        || search(x, y - 1, h, w, blocks, seen);
}
