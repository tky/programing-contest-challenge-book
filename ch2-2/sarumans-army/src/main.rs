fn main() {
    let result = solve(&[1, 7, 15, 20, 30, 50], 10);
    println!("{:?}", result);
}

fn solve(xs: &[usize], r: usize) -> Vec<usize> {
    let mut result = vec![];
    let mut i = 0;
    loop {
        let j = last_index_within_limit(xs, i, r);
        i = last_index_within_limit(xs, j, r) + 1;
        result.push(j);
        if i >= xs.len() {
            break;
        }
    }
    result
}

fn last_index_within_limit(xs: &[usize], x: usize, r: usize) -> usize {
    let limit = xs[x] + r;
    let mut index = xs.len() - 1;
    // ここを二分探索すると性能改善できると思われる
    while xs[index] > limit {
        index -= 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_index_within_limit_basic() {
        // xs[x] + r 以下の最大要素を、x番目以降から探す。
        // xs = [1, 7, 15, 20, 30, 50], r = 10
        // 例1: x=0 => limit = 1 + 10 = 11
        //       xs[0..] = [1, 7, 15, 20, 30, 50]
        //       15 は 11 を超えるのでNG、最大は 7 => インデックス1
        let xs = vec![1, 7, 15, 20, 30, 50];
        let r = 10;
        let x = 0;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 1, "xs[1] = 7 は 11 以下、xs[2] = 15 は 11 超 => index=1 が正");

        // 例2: x=1 => limit = 7 + 10 = 17
        //       xs[1..] = [7, 15, 20, 30, 50]
        //       20 は 17 を超えるのでNG、最大は 15 => インデックス2
        let x = 1;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 2, "xs[2] = 15 は 17 以下、xs[3] = 20 は 17 超 => index=2 が正");

        // 例3: x=2 => limit = 15 + 10 = 25
        //       xs[2..] = [15, 20, 30, 50]
        //       30 は 25 を超える => 最大は 20 => インデックス3
        let x = 2;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 3, "xs[3] = 20 は 25 以下、xs[4] = 30 は 25 超 => index=3 が正");

        // 例4: x=3 => limit = 20 + 10 = 30
        //       xs[3..] = [20, 30, 50]
        //       30 は 30 以下 => インデックス4
        let x = 3;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 4, "xs[4] = 30 は 30 以下、xs[5] = 50 は 30 超 => index=4 が正");
    }

    #[test]
    fn test_last_index_within_limit_edge() {
        // 端の要素付近をテスト
        let xs = vec![1, 5, 10, 15];
        let r = 5;

        // x=3 => limit = 15 + 5 = 20
        // xs[3..] = [15], 15 <= 20 なので index=3
        let x = 3;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 3);

        // x=2 => limit = 10 + 5 = 15
        // xs[2..] = [10, 15], 15 <= 15 なので index=3
        let x = 2;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_last_index_within_limit_all_in_range() {
        // x番目以降のすべてが limit 以下のケース
        let xs = vec![2, 4, 6, 8];
        let r = 10;

        // x=1 => limit = 4 + 10 = 14
        // xs[1..] = [4, 6, 8], 全て14以下 => index=3 (配列長-1)
        let x = 1;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 3, "x=1 から最後まで全てカバー可能");

        // x=0 => limit = 2 + 10 = 12
        // xs[0..] = [2, 4, 6, 8], 全て12以下 => index=3
        let x = 0;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_last_index_within_limit_no_element_in_range() {
        // x番目の要素自体が limit を超えてしまうケース
        // ただし通常は「xs[x] <= xs[x] + r」が必ず真なので、発生しづらいシナリオです。
        // もし仕様として、xs[x] > xs[x] + r があり得るとすればテストでどうなるか確認
        // ここではあえて x=0 で不整合を起こすデータを作り、想定通りに動くかテスト
        let xs = vec![50, 60, 70];
        let r = 10;
        // x=0 => limit= 50 + 10 = 60
        // xs[0..] = [50, 60, 70]
        // 70 は 60 を超える => xs[2] NG、ただし 60 は <= 60 => インデックス1
        // ここでは「x 番目は必ず含む」と想定するなら 0 は範囲内だが、最大は 1
        let x = 0;
        let index = last_index_within_limit(&xs, x, r);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_last_index_within_limit_single_element() {
        // 要素が1つだけの場合
        let xs = vec![10];
        let r = 5;
        // x=0 => limit = 10 + 5 = 15
        // xs[0] = 10 <= 15 なので index=0
        let index = last_index_within_limit(&xs, 0, r);
        assert_eq!(index, 0);
    }

    /// 各印のカバー範囲は [mark - r, mark + r] とする。
    /// xs の各点が、どれか1つの印のカバー範囲に含まれているかを判定する。
    fn is_all_covered(xs: &[usize], r: usize, marks: &[usize]) -> bool {
        xs.iter().all(|&pos| {
            marks.iter().any(|&m| {
                let mark_pos = xs[m];
                let left_bound = mark_pos.saturating_sub(r);
                let right_bound = mark_pos + r;
                pos >= left_bound && pos <= right_bound
            })
        })
    }

    #[test]
    fn test_solve_single_point() {
        let xs = vec![10];
        let r = 5;
        let result = solve(&xs, r);
        // 単一要素なら [0] が返るはず
        assert_eq!(result, vec![0], "単一要素の場合は [0] になる");
        assert!(is_all_covered(&xs, r, &result), "単一点は左右カバー範囲に含まれる");
    }

    #[test]
    fn test_solve_all_points_covered_by_one_mark() {
        // すべての点が1つの印で左右両側に十分カバーできるケース
        let xs = vec![2, 3, 4, 5];
        let r = 10;
        let result = solve(&xs, r);
        assert!(is_all_covered(&xs, r, &result), "全ての点がカバーされている必要があります");
        // 最適なら印は1つで十分
        assert_eq!(result.len(), 1, "1つの印で全点をカバーできるはず");
    }

    #[test]
    fn test_solve_poj3069_typical_example() {
        // Saruman’s Army でよく知られる例
        let xs = vec![1, 7, 15, 20, 30, 50];
        let r = 10;
        let result = solve(&xs, r);
        assert!(is_all_covered(&xs, r, &result), "全ての点がカバーされている必要があります");
        // 一般的な貪欲法の解では 3 箇所でカバーできる例が多いので、印の数が 3 であることを確認
        assert_eq!(result.len(), 3, "この例では印3箇所で全点カバーできるはず");
    }

    #[test]
    fn test_solve_multiple_marks_needed() {
        // 点が離れて配置され、複数回印を置く必要があるケース
        let xs = vec![1, 2, 10, 11, 20, 21];
        let r = 2;
        let result = solve(&xs, r);
        assert!(is_all_covered(&xs, r, &result), "全ての点が左右のカバー範囲でカバーされている必要があります");
        // ここでは印が必要最低限配置されているかより、すべての点がカバーされているかを重視
        assert!(!result.is_empty(), "少なくとも1箇所は印が必要");
    }

    #[test]
    fn test_solve_already_sorted_but_spread_out() {
        // 点の間隔が広く、r が小さいケース
        let xs = vec![5, 10, 15, 25, 26, 27];
        let r = 3;
        let result = solve(&xs, r);
        assert!(is_all_covered(&xs, r, &result), "全ての点が左右カバー範囲でカバーされていなければなりません");
    }
}
