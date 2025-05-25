fn main() {
    println!("Hello, world!");
    let result = solve(3, 0.75, 600000);
    println!("{:.6}", result); // => 0.843750
}

fn solve(m: usize, p: f64, x: usize) -> f64 {
    const MAX_M: usize = 15;

    let n = 1 << m;
    let mut dp = vec![vec![0.0f64; (1 << MAX_M) + 1]; 2];

    let (left, right) = dp.split_at_mut(1);
    let mut prv = &mut left[0];
    let mut nxt = &mut right[0];

    prv[n] = 1.0;

    for _ in 0..m {
        for i in 0..=n {
            let jub = std::cmp::min(i, n - i);
            let mut t = 0.0;
            for j in 0..=jub {
                let win = i + j;
                let lose = i - j;
                let val = p * prv[win] + (1.0 - p) * prv[lose];
                if val > t {
                    t = val;
                }
            }
            nxt[i] = t;
        }
        std::mem::swap(&mut prv, &mut nxt);
    }

    let i = x * n / 1_000_000;
    prv[i]
}
