use union_find::{QuickUnionUf, UnionBySize, UnionFind};

struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

/*
for each edge e in E' (昇順ソート済み):
    let (u, v) = endpoints of e
    if find(u) ≠ find(v):    # u と v がまだ同じ木に属していなければ
        T ← T ∪ {e}          # e を MST に追加
        union(u, v)          # u の木と v の木を併合
    end if
    if |T| = n - 1:          # 必要な辺数に達したら終了
        break
    end if
end for
 */
fn resolve(n: usize, edges: &mut [Edge]) -> Option<usize> {
    edges.sort_by_key(|edge| edge.cost);
    let mut uf = QuickUnionUf::<UnionBySize>::new(n);

    let mut total_cost = 0;

    edges.iter().for_each(|edge| {
        if uf.find(edge.from) != uf.find(edge.to) {
            total_cost += edge.cost;
            uf.union(edge.from, edge.to);
        }
    });

    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut edges = vec![
            Edge {
                from: 0,
                to: 1,
                cost: 2,
            },
            Edge {
                from: 0,
                to: 2,
                cost: 5,
            },
            Edge {
                from: 2,
                to: 1,
                cost: 4,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 2,
            },
            Edge {
                from: 1,
                to: 3,
                cost: 6,
            },
            Edge {
                from: 1,
                to: 4,
                cost: 10,
            },
            Edge {
                from: 3,
                to: 5,
                cost: 1,
            },
            Edge {
                from: 4,
                to: 5,
                cost: 3,
            },
            Edge {
                from: 4,
                to: 6,
                cost: 5,
            },
            Edge {
                from: 5,
                to: 6,
                cost: 9,
            },
        ];
        assert_eq!(resolve(7, &mut edges), Some(17));
    }

    #[test]
    fn test_simple_graph() {
        let mut edges = vec![
            Edge {
                from: 0,
                to: 1,
                cost: 1,
            },
            Edge {
                from: 0,
                to: 2,
                cost: 5,
            },
            Edge {
                from: 1,
                to: 2,
                cost: 3,
            },
            Edge {
                from: 1,
                to: 3,
                cost: 4,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 2,
            },
        ];
        // MST の辺は cost=1,2,3 の3本 → 合計 6
        assert_eq!(resolve(4, &mut edges), Some(6));
    }
}

fn main() {
    println!("Hello, world!");
    let mut edges = vec![
        Edge {
            from: 0,
            to: 1,
            cost: 2,
        },
        Edge {
            from: 0,
            to: 2,
            cost: 5,
        },
        Edge {
            from: 2,
            to: 1,
            cost: 4,
        },
        Edge {
            from: 2,
            to: 3,
            cost: 2,
        },
        Edge {
            from: 1,
            to: 3,
            cost: 6,
        },
        Edge {
            from: 1,
            to: 4,
            cost: 10,
        },
        Edge {
            from: 3,
            to: 5,
            cost: 1,
        },
        Edge {
            from: 4,
            to: 5,
            cost: 3,
        },
        Edge {
            from: 4,
            to: 6,
            cost: 5,
        },
        Edge {
            from: 5,
            to: 6,
            cost: 9,
        },
    ];
    resolve(7, &mut edges);
}
