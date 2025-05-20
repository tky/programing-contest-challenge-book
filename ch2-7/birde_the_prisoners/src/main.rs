fn main() {
    let ans = solve(20, &[3, 6, 14]);
    println!("ans = {}", ans);
}

fn solve(p: usize, a: &[usize]) -> usize {
    let mut v = [0].to_vec();
    v.extend(a);
    v.push(p + 1);
    let n = v.len();
    let mut dp = vec![vec![0; n]; n];

    for width in 2..v.len() {
        for i in 0..v.len() - width {
            let j = i + width;
            let bribe = v[j] - v[i] - 2;
            dp[i][j] = usize::MAX;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + bribe);
            }
        }
    }
    dp[0][v.len() - 1]
}
