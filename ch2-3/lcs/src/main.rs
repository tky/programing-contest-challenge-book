use std::cmp::max;

fn main() {
    let s: Vec<char> = "AGGTAB".chars().collect();
    let t: Vec<char> = "GXTXAYB".chars().collect();
    println!("{}", solve(&s, &t));
}


fn solve(s: &[char], t: &[char]) -> usize {
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 1..s.len() + 1 {
        for j in 1..t.len() + 1 {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[s.len()][t.len()]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_same() {
        let s: Vec<char> = "a".chars().collect();
        let t: Vec<char> = "a".chars().collect();
        assert_eq!(solve(&s, &t), 1);
    }

    #[test]
    fn test_single_char_diff() {
        let s: Vec<char> = "a".chars().collect();
        let t: Vec<char> = "b".chars().collect();
        assert_eq!(solve(&s, &t), 0);
    }

    #[test]
    fn test_abcd_becd() {
        let s: Vec<char> = "abcd".chars().collect();
        let t: Vec<char> = "becd".chars().collect();
        assert_eq!(solve(&s, &t), 3);
    }

    #[test]
    fn test_abcdef_ace() {
        let s: Vec<char> = "abcdef".chars().collect();
        let t: Vec<char> = "ace".chars().collect();
        assert_eq!(solve(&s, &t), 3);
    }

    #[test]
    fn test_aggtab_gxtxayb() {
        let s: Vec<char> = "AGGTAB".chars().collect();
        let t: Vec<char> = "GXTXAYB".chars().collect();
        assert_eq!(solve(&s, &t), 4);
    }
}