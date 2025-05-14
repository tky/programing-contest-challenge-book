fn main() {
    println!("Hello, world!");
    solve(&mut vec![1, 3, 5], &mut vec![2, 4, 6]);
}

fn solve(v1: &mut [i64], v2: &mut [i64]) -> i64 {
    v1.sort_by(|a, b| a.cmp(b));
    v2.sort_by(|a, b| b.cmp(a));
    v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut v1 = vec![1, 3, -5];
        let mut v2 = vec![-2, 4, 1];
        assert_eq!(solve(&mut v1, &mut v2), -25);
    }

    #[test]
    fn test_solve_with_max() {
        let mut v1 = vec![100000];
        let mut v2 = vec![100000];
        assert_eq!(solve(&mut v1, &mut v2), 100000 * 100000);
    }
}
