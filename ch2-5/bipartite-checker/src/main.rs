fn main() {
    println!("Hello, world!");
}

fn resolve(graph: &[Vec<usize>]) -> bool {
    // 0, 1, 2
    // 0: not visited
    let mut color = vec![0 as usize; graph.len()];

    for i in 0..graph.len() {
        // not visited yet
        if color[i] == 0 {
            let nodes = &graph[i];
            color[i] = 1;
            if nodes.iter().any(|node| color[*node] == 1) {
                return false;
            }
            nodes.iter().for_each(|node| color[*node] = 2);
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
        assert_eq!(resolve(&graph), true);
    }

    #[test]
    fn test_resolve2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(resolve(&graph), true);
    }
}
