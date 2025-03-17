use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

fn solve(a: &[usize]) -> usize {
    let mut dp = vec![1 as usize; a.len()];
    for i in 0..a.len() {
        for j in 0..i {
            if a[i] > a[j] {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(&[4, 2, 3, 1, 5]), 3);
    }

    #[test]
    fn test_increasing_sequence() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(solve(&a), 5);
    }

    #[test]
    fn test_decreasing_sequence() {
        let a = vec![5, 4, 3, 2, 1];
        assert_eq!(solve(&a), 1);
    }

    #[test]
    fn test_mixed_sequence() {
        let a = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
        assert_eq!(solve(&a), 6);
    }
}
