use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cost: u32,
}

const INF: u32 = std::u32::MAX;

/// グラフは隣接リスト。0 から n-1 までのノードがある前提。
/// 返り値は dist2[n-1]（0→n-1 の二番手最短距離）
fn second_shortest_path(n: usize, graph: &[Vec<Edge>]) -> u32 {
    // dist[v]  = v への「最短距離」
    // dist2[v] = v への「二番目に短い距離」
    let mut dist = vec![INF; n];
    let mut dist2 = vec![INF; n];

    // min-heap には (距離, 頂点) を Reverse で入れる
    let mut pq = BinaryHeap::new();

    // スタートは 0→距離0
    dist[0] = 0;
    pq.push(Reverse((0u32, 0usize)));

    while let Some(Reverse((d, v))) = pq.pop() {
        // 既に v への二番手距離がこれより短ければスキップ
        if dist2[v] < d {
            continue;
        }
        // v から伸びる全辺をチェック
        for e in &graph[v] {
            let mut d2 = d.saturating_add(e.cost);

            // ── (1) 最短距離を更新できるか？
            if d2 < dist[e.to] {
                std::mem::swap(&mut dist[e.to], &mut d2);
                pq.push(Reverse((dist[e.to], e.to)));
            }

            // ── (2) d2 が “最短” より長く、“二番目” を更新できる範囲か？
            if dist[e.to] < d2 && d2 < dist2[e.to] {
                dist2[e.to] = d2;
                pq.push(Reverse((dist2[e.to], e.to)));
            }
        }
    }
    dist2[n - 1]
}

#[cfg(test)]
mod tests {
    use super::{Edge, INF, second_shortest_path};

    fn make_graph(n: usize, edges: &[(usize, usize, u32)]) -> Vec<Vec<Edge>> {
        let mut g = vec![Vec::new(); n];
        for &(u, v, w) in edges {
            g[u].push(Edge { to: v, cost: w });
            g[v].push(Edge { to: u, cost: w });
        }
        g
    }

    #[test]
    fn single_node_has_no_second_path() {
        let graph: Vec<Vec<Edge>> = vec![Vec::new()];
        // ノード０→０ のウォークは距離0 のみで、次は存在しないので INF
        assert_eq!(second_shortest_path(1, &graph), INF);
    }

    #[test]
    fn two_nodes_cycle_walk() {
        let graph = make_graph(2, &[(0, 1, 7)]);
        // shortest = 7 (0→1)
        // second  = 7+7+7 = 21 (0→1→0→1)
        assert_eq!(second_shortest_path(2, &graph), 21);
    }

    #[test]
    fn three_nodes_simple_triangle() {
        // 0–1:1, 1–2:1, 0–2:3
        // shortest = 2 (0→1→2)
        // second  = 3 (0→2)
        let graph = make_graph(3, &[(0, 1, 1), (1, 2, 1), (0, 2, 3)]);
        assert_eq!(second_shortest_path(3, &graph), 3);
    }

    #[test]
    fn four_nodes_multiple_paths() {
        // 0→1:1, 0→2:4, 1→2:2, 1→3:5, 2→3:1
        // shortest = 4 (0→1→2→3)
        // second  = 5 (0→2→3)
        let graph = make_graph(4, &[(0, 1, 1), (0, 2, 4), (1, 2, 2), (1, 3, 5), (2, 3, 1)]);
        assert_eq!(second_shortest_path(4, &graph), 5);
    }

    #[test]
    fn unreachable_destination() {
        // 0–1:1 のみ。2 へはそもそも到達不能 → INF
        let graph = make_graph(3, &[(0, 1, 1)]);
        assert_eq!(second_shortest_path(3, &graph), INF);
    }
}

fn main() {
    // Example usage
    let graph = vec![
        vec![Edge { to: 1, cost: 1 }, Edge { to: 2, cost: 4 }],
        vec![
            Edge { to: 0, cost: 1 },
            Edge { to: 2, cost: 2 },
            Edge { to: 3, cost: 5 },
        ],
        vec![
            Edge { to: 0, cost: 4 },
            Edge { to: 1, cost: 2 },
            Edge { to: 3, cost: 1 },
        ],
        vec![Edge { to: 1, cost: 5 }, Edge { to: 2, cost: 1 }],
    ];
    let n = graph.len();
    let result = second_shortest_path(n, &graph);
    println!(
        "The second shortest path from node 0 to node {} is {}",
        n - 1,
        result
    );
}
