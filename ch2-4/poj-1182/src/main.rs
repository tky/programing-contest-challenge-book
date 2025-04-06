use union_find::{QuickUnionUf, UnionBySize, UnionFind};

enum Type {
    Type1,
    Type2,
}

fn main() {
    let mut uf = QuickUnionUf::<UnionBySize>::new(10);

    for i in 0..10 {
        assert_eq!(uf.find(i), i);
    }

    uf.union(0, 1);

    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.find(1), 0);
    assert_eq!(uf.find(1), uf.find(0));
}

fn resolve(n: usize, informations: &[(Type, usize, usize)]) -> Vec<usize> {
    let mut uf = QuickUnionUf::<UnionBySize>::new(n * 3);
    let mut ans = vec![];

    for i in 0..informations.len() {
        let info = &informations[i];

        let (t, x, y) = info;
        let x = *x;
        let y = *y;

        if x >= n || y >= n {
            ans.push(i);
            continue;
        }

        match t {
            Type::Type1 => {
                // xがAでyがB
                if uf.find(x) == uf.find(y + n) {
                    ans.push(i);
                // xがAでyがC
                } else if uf.find(x) == uf.find(y + 2 * n) {
                    ans.push(i);
                } else {
                    // xがAならyもA
                    uf.union(x, y);
                    // xがBならyもB
                    uf.union(x + n, y + n);
                    // xがCならyもC
                    uf.union(x + 2 * n, y + 2 * n);
                }
            }
            Type::Type2 => {
                // xとyが同じ
                if uf.find(x) == uf.find(y) {
                    ans.push(i);
                // 逆に食われる関係
                } else if uf.find(x) == uf.find(y + 2 * n) {
                    ans.push(i);
                } else {
                    // xがAならyはB
                    uf.union(x, y + n);
                    // xがBならyはC
                    uf.union(x + n, y + 2 * n);
                    // xがCならyはA
                    uf.union(x + 2 * n, y);
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let n = 100;
        let informations = vec![
            (Type::Type1, 101, 1),
            (Type::Type2, 1, 2),
            (Type::Type2, 2, 3),
            (Type::Type2, 3, 3),
            (Type::Type1, 1, 3),
            (Type::Type2, 3, 1),
            (Type::Type1, 5, 5),
        ];
        let result = resolve(n, &informations);
        assert_eq!(result, vec![0, 3, 4]);
    }
}
