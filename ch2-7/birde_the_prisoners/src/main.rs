fn main() {
    let ans = solve(20, &[3, 6, 14]);
    println!("ans = {}", ans);
}

fn solve(p: usize, a: &[usize]) -> usize {
    let mut v = [0].to_vec();
    v.extend(a);
    v.push(p + 1);
    let mut dp = vec![vec![0; v.len() + 1]; v.len() + 1];

    for width in 2..v.len() {
        for i in 0..v.len() - width {
            let j = i + width;
            dp[i][j] = usize::MAX;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + (v[j] - v[i] - 2));
            }
        }
    }
    dp[0][v.len() - 1]
}
