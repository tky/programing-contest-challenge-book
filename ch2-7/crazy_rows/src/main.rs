fn main() {
    println!("Hello, world!");
    solve(&vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]]);
}

fn solve(matrix: &[Vec<i32>]) -> usize {
    let n = matrix.len();
    // 各行の右端の1の位置を記録する
    let mut rops: Vec<usize> = matrix
        .iter()
        .map(|row| row.iter().rposition(|&x| x == 1).unwrap_or(0))
        .collect();

    println!("{:?}", rops);

    // このような行列を考える
    // 0 0 1
    // 1 0 0
    // 0 1 0
    //
    // rpos = [2, 0 , 1]
    //
    // 0行目に置けるrposs[i] <= 0 を満たすのはi = 1なので
    // 1行目を0行目に移す
    // 次に1行目に置けるrposs[i] <= 1を満たすのはi = 2なので
    // 2行目を1行目に移す

    let mut swaps = 0;

    for r in 0..n {
        // kはr行目に挿入できる最初の行
        let k = (r..n).find(|&k| rops[k] <= r).expect("Invalid matrix");
        // 隣接swapでkにある行をrへ持っていくのでk -r回swapが必要
        swaps += k - r;

        let v = rops.remove(k);
        rops.insert(r, v);
    }
    swaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let matrix = vec![vec![1, 0], vec![1, 1]];
        assert_eq!(solve(&matrix), 0);

        let matrix = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]];
        assert_eq!(solve(&matrix), 2);

        let matrix = vec![
            vec![1, 1, 1, 0],
            vec![1, 1, 0, 0],
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 0],
        ];
        assert_eq!(solve(&matrix), 4);
    }
}
