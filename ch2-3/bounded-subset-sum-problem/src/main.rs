fn main() {
    println!("Hello, world!");
}

fn solve(k: usize, vs: &[(usize, usize)]) -> bool {
    // dp[i][j] = Union (dp[i - 1][j - k + a_i])
    let mut dp = vec![vec![false; k + 1]; vs.len() + 1];
    dp[0][0] = true;
    for i in 1..vs.len() + 1 {
        for j in 0..k + 1 {
            let (n, m) = vs[i - 1];
            for k in 0..m + 1 {
                let prev = (j as isize) - n as isize * k as isize;
                if prev < 0 {
                    break;
                }
                if dp[i - 1][prev as usize] {
                    dp[i][j] = true;
                    break;
                }
            }
        }
    }
    dp[vs.len()][k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_true() {
        assert_eq!(solve(10, &[(3, 5), (4, 2)]), true);
    }

    #[test]
    fn test_basic_false() {
        assert_eq!(solve(5, &[(3, 5), (4, 2)]), false);
    }

    #[test]
    fn test_empty_vs() {
        assert_eq!(solve(0, &[]), true);
        assert_eq!(solve(1, &[]), false);
    }

    #[test]
    fn test_single_value() {
        assert_eq!(solve(3, &[(3, 1)]), true);
        assert_eq!(solve(6, &[(3, 1)]), false);
    }

    #[test]
    fn test_complex_case() {
        assert_eq!(solve(10, &[(2, 3), (3, 2)]), true);
    }
}
