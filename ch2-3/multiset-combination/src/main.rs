use std::cmp::min;

fn main() {
    println!("Hello, world!");
}

fn solve(vs: &[usize], m: usize) -> usize {
    // dp[i][j] := 1番目からi番目までの品物を使って合計こ個を選ぶ方法の数
    // dp[i][j] = for (k = 0 to min(vs[i], j) dp[i - 1][j - k]
    // 0番目の品物を使って合計0個を選ぶ方法は1とおり
    // dp[0][0] = 1
    // 0番目の品物を使って合計j個を選ぶ方法は0通り(そんな方法はありえない）
    // dp[0][j] = 0 (j > 0)
    let mut dp = vec![vec![0; m + 1]; vs.len() + 1];
    dp[0][0] = 1;

    for i in 1..vs.len() + 1 {
        for j in 0..m + 1 {
            dp[i][j] = (0..=min(vs[i - 1], j)).map(|k| dp[i - 1][j - k]).sum();
        }
    }
    dp[vs.len()][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let vs = vec![1, 2, 3];
        let m = 3;
        assert_eq!(solve(&vs, m), 6);
    }

    #[test]
    fn test_zero_selection() {
        let vs = vec![1, 2, 3];
        let m = 0;
        assert_eq!(solve(&vs, m), 1);
    }

    #[test]
    fn test_single_product() {
        let vs = vec![5];
        assert_eq!(solve(&vs, 3), 1);
        assert_eq!(solve(&vs, 5), 1);
        assert_eq!(solve(&vs, 6), 0);
    }

    #[test]
    fn test_two_products() {
        let vs = vec![2, 2];
        assert_eq!(solve(&vs, 1), 2);
        assert_eq!(solve(&vs, 2), 3);
        assert_eq!(solve(&vs, 3), 2);
        assert_eq!(solve(&vs, 4), 1);
    }
}
