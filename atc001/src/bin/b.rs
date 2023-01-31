use proconio::*;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    solve(n, q);
}

fn solve(n: usize, q: usize) {
    let mut uf = UnionFind::new(n);

    for _i in 0..q {
        input! {
            p: usize,
            a: usize,
            b: usize,
        }

        match p {
            0 => uf.unite(a, b),
            1 => {
                if uf.same(a, b) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => (),
        }
    }
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
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
