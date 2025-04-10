use std::vec;

struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

fn resolve(n: usize, edges: &[Edge]) -> usize {
    let mut dist = vec![usize::MAX; n];
    // 0を開始点として固定
    dist[0] = 0;

    // 最初のループでは0と隣接しているノードの最短距離を取得可能
    // 2回目以降のループでは、最短距離が確定しているノードから隣接ノードの最短距離を更新する
    // 以下全てのノードの最短距離が確定するまでループを回す
    for _ in 0..(n - 1) {
        for edge in edges {
            // from → to
            if dist[edge.from] != usize::MAX && dist[edge.to] > dist[edge.from] + edge.cost {
                dist[edge.to] = dist[edge.from] + edge.cost;
            }
            // to → from（無向グラフとしての逆方向）
            if dist[edge.to] != usize::MAX && dist[edge.from] > dist[edge.to] + edge.cost {
                dist[edge.from] = dist[edge.to] + edge.cost;
            }
        }
    }

    dist[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let edges = vec![
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
        assert_eq!(resolve(7, &edges), 16);
    }
}
