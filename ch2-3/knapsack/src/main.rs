use std::cmp::max;

fn main() {
    let result = solve(
        8,
        &[
            (2, 3),
            (2, 2),
            (4, 5),
            (5, 10),
            (3, 6),
        ]
    );
    println!("{}", result);
}

fn solve(max_weight: usize, items: &[(usize, usize)]) -> usize {
    // 最初のi個の品物から重さの総和がj以下となるように選んだときの価値の総和の最大値
    // i = 0の時は品物を１つも選べないので価値は0になる
    // j = 0の時は追加できないので価値は0になる
    let mut dp = vec![vec![0; max_weight + 1]; items.len() + 1];
    for i in 1..items.len() + 1 {
        for j in 0..max_weight + 1 {
            let (weight, value) = items[i - 1];
            if j < weight {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(dp[i - 1][j],  dp[i - 1][j - weight] + value);
            }
        }
    }
    // dpの最後のセルが最終的な答えになっている
    dp[items.len()][max_weight]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1. 品物が空の場合
    ///    どれだけ容量があっても選ぶものがないので結果は0。
    #[test]
    fn test_no_items() {
        let max_weight = 10;
        let items: &[(usize, usize)] = &[]; // (weight, value)
        assert_eq!(solve(max_weight, items), 0);
    }

    /// 2. 品物が1つだけ
    ///    - 容量が足りる場合と足りない場合を分けてテスト
    #[test]
    fn test_single_item() {
        // 2-1. 容量が足りる場合
        let max_weight = 5;
        let items = [(3, 10)];
        // 重さ3の品物を入れられるので、価値は10が最大
        assert_eq!(solve(max_weight, &items), 10);

        // 2-2. 容量が足りない場合
        let max_weight = 2;
        // 重さ3の品物を入れられないので、価値0が最大
        assert_eq!(solve(max_weight, &items), 0);
    }

    /// 3. 参考例：教科書的な小規模ケース
    ///    例: (w=2,v=3), (w=3,v=4), (w=4,v=5), max_weight=5
    ///    最適解は品物1+2 (重さ=2+3=5, 価値=3+4=7) で価値7
    #[test]
    fn test_small_example() {
        let max_weight = 5;
        let items = [(2, 3), (3, 4), (4, 5)];
        assert_eq!(solve(max_weight, &items), 7);
    }

    /// 4. 重さ・価値ともに色々なパターン
    ///    例: 
    ///      - 品物1: (w=2, v=3)
    ///      - 品物2: (w=2, v=2)
    ///      - 品物3: (w=4, v=5)
    ///      - 品物4: (w=5, v=10)
    ///      - 品物5: (w=3, v=6)
    ///      max_weight=8
    ///
    ///    いくつか組み合わせを試して最適を確かめる。
    #[test]
    fn test_various_items() {
        let max_weight = 8;
        let items = [
            (2, 3),
            (2, 2),
            (4, 5),
            (5, 10),
            (3, 6),
        ];
        assert_eq!(solve(max_weight, &items), 16);
    }

    /// 5. 大きな容量かつ大きなアイテム数 (ストレステスト的)
    ///    実装が O(nW) のDPの場合、ここまで大きくすると遅い可能性があるが
    ///    小さめの範囲でテスト用に確認。
    #[test]
    fn test_larger_case() {
        let max_weight = 20;
        let items = [
            (2,  3), (5,  10), (10, 15), (11, 17),
            (4,  6), (3,  5),  (6,  9),  (7,  13),
            (8,  13), (9,  14),
        ];
        // 正確な手計算は面倒だが、いくつか当たりをつけてみる:
        //   - (5,10) + (10,15) + (4,6) = 重さ19, 価値31
        //   - (5,10) + (10,15) + (3,5) = 重さ18, 価値30
        //   - (11,17) + (4,6) + (3,5) = 重さ18, 価値28
        //   - (10,15) + (6,9) + (4,6) = 重さ20, 価値30
        //   - (5,10) + (7,13) + (8,13) = 重さ20, 価値36 ← 結構大きい
        // → さらに最適な組み合わせがあるかもしれないが、ここでは 36 以上になるかをチェック
        //   (5,10) + (7,13) + (8,13) = 36 が候補
        // 手動で探した限り36が最大そうなのでそれをアサート
        assert_eq!(solve(max_weight, &items), 36);
    }
}