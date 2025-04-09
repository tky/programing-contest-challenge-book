fn main() {
    println!("Hello, world!");
}

fn resolve(graph: &[Vec<usize>]) -> bool {
    // 0, 1, 2
    // 0: not visited
    let mut color = vec![0; graph.len()];

    for i in 0..graph.len() {
        // not visited yet
        if color[i] == 0 {
            if !dfs(graph, &mut color, i, 1) {
                return false;
            }
        }
    }
    true
}

fn dfs(graph: &[Vec<usize>], colors: &mut [i32], node: usize, color: i32) -> bool {
    colors[node] = color;

    for &neighbor in &graph[node] {
        if colors[neighbor] == color {
            return false;
        }

        if colors[neighbor] == 0 && !dfs(graph, colors, neighbor, -color) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(resolve(&graph), false);
    }

    #[test]
    fn test_resolve2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(resolve(&graph), true);
    }

    #[test]
    fn test_non_bipartite() {
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(resolve(&graph), false);
    }

    #[test]
    fn test_bipartite() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(resolve(&graph), true);
    }
}
