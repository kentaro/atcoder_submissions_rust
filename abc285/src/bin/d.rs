use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
      n: usize,
    }

    solve(n)
}

fn solve(n: usize) {
    let mut indices: HashMap<String, usize> = HashMap::new();
    let mut uf: UnionFind = UnionFind::new(n * 2);
    let mut index: usize = 0;

    for _i in 0..n {
        input! {
          s: String,
          t: String,
        }

        let s_index: usize;
        if indices.contains_key(&s) {
            s_index = indices[&s];
        } else {
            s_index = index;
            indices.insert(s, s_index);
            index += 1;
        }

        let t_index: usize;
        if indices.contains_key(&t) {
            t_index = indices[&t];
        } else {
            t_index = index;
            indices.insert(t, t_index);
            index += 1;
        }

        if uf.same(s_index, t_index) {
            println!("No");
            return;
        } else {
            uf.unite(s_index, t_index);
        }
    }

    println!("Yes")
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.root(self.par[x]);
            self.par[x] = r;
            r
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}
