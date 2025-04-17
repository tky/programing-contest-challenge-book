use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
    let graph: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4]];
    resolve(&graph);
}

fn resolve(graph: &[Vec<u32>]) -> u32 {
    // Minimum Spanning Tree (MST)
    let mut mst = vec![false; graph.len()];
    let mut result = 0;
    let mut heap = BinaryHeap::new();

    graph[0].iter().enumerate().for_each(|(v, &cost)| {
        if cost > 0 {
            heap.push(Reverse((cost, v)));
        }
    });
    mst[0] = true;

    while mst.iter().any(|&x| !x) {
        match heap.pop() {
            Some(Reverse((cost, v))) => {
                if mst[v] {
                    continue;
                }
                result += cost;
                mst[v] = true;
                graph[v].iter().enumerate().for_each(|(u, &cost)| {
                    if cost > 0 {
                        heap.push(Reverse((cost, u)));
                    }
                })
            }
            _ => break,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let graph: Vec<Vec<u32>> = vec![
            vec![0, 2, 0, 6, 0],
            vec![2, 0, 3, 8, 5],
            vec![0, 3, 0, 0, 7],
            vec![6, 8, 0, 0, 9],
            vec![0, 5, 7, 9, 0],
        ];
        assert_eq!(resolve(&graph), 16);
    }
}
