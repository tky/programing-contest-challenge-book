fn main() {
    println!("Hello, world!");
}

fn solve(max_weight: usize, items: &[(usize, usize)]) -> usize {
    let mut dp = vec![vec![0; max_weight + 1]; items.len() + 1];
    for i in 1..items.len() + 1 {
        for w in 1..max_weight + 1 {
            let (weight, value) = items[i - 1];
            if w < weight {
                dp[i][w] = dp[i - 1][w];
            } else {
                // 制限ありだとi - 1だったが、制限なしだとiになる
                // dp[i][w] = std::cmp::max(dp[i - 1][w], dp[i - 1][w - weight] + value);
                dp[i][w] = std::cmp::max(dp[i - 1][w], dp[i][w - weight] + value);
            }
        }
    }
    dp[items.len()][max_weight]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_items() {
        let max_weight = 10;
        let items: &[(usize, usize)] = &[];
        assert_eq!(solve(max_weight, items), 0);
    }

    #[test]
    fn test_zero_weight() {
        let max_weight = 0;
        let items = [(1, 2), (2, 3)];
        assert_eq!(solve(max_weight, &items), 0);
    }

    #[test]
    fn test_single_item() {
        let max_weight = 10;
        let items = [(3, 10)];
        assert_eq!(solve(max_weight, &items), 30);
    }

    #[test]
    fn test_two_items() {
        let max_weight = 7;
        let items = [(2, 3), (3, 4)];
        assert_eq!(solve(max_weight, &items), 10);
    }

    #[test]
    fn test_two_items_reversed() {
        let max_weight = 7;
        let items = [(3, 4), (2, 3)];
        assert_eq!(solve(max_weight, &items), 10);
    }

    #[test]
    fn test_complex() {
        let max_weight = 8;
        let items = [(2, 3), (2, 2), (4, 5), (5, 10), (3, 6)];
        assert_eq!(solve(max_weight, &items), 16);
    }
}
