use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let boards = vec![8, 5, 8];
    let cost = resolve(&boards);
    println!("{}", cost);
}


// この問題は「最適マージパターン問題」とも呼ばれる問題に似ており、貪欲法によって最適解が得られることが知られています。具体的な手順は以下の通りです。
// 初期化：
// すべての板の長さを最小ヒープ（優先度付きキュー）に挿入します。
// 最小ヒープを使うことで、常に最小の要素（最も短い板）を効率的に取り出すことができます。
// 結合操作の繰り返し：
// 板が1本になるまで、以下の操作を繰り返します。

// ヒープから最も短い2本の板を取り出す。
// この2本を結合し、その和を結合コストとして累積の合計コストに加える。
// 結合した新しい板（長さは取り出した2本の和）をヒープに戻す。
// 終了：
// ヒープに残る板が1本になった時点で、累積した合計コストが解となります。
fn resolve(ls: &[usize]) -> usize {
    let mut min_heap = BinaryHeap::new();
    let mut cost = 0;
    for &l in ls {
        min_heap.push(Reverse(l));
    }

    while min_heap.len() > 1 {
        let Reverse(l1) = min_heap.pop().unwrap();
        let Reverse(l2) = min_heap.pop().unwrap();
        let new_board = l1 + l2;
        cost += new_board;
        min_heap.push(Reverse(new_board));
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        // 板がない場合はコスト0となるはずです
        let boards: Vec<usize> = vec![];
        assert_eq!(resolve(&boards), 0);
    }

    #[test]
    fn test_single_board() {
        // 板が1本だけなら、修理（結合）の必要がないのでコストは0です
        let boards = vec![10];
        assert_eq!(resolve(&boards), 0);
    }

    #[test]
    fn test_example1() {
        // 例: [8, 5, 8]
        // 最適な結合方法:
        // 5 + 8 = 13 (コスト 13)
        // 13 + 8 = 21 (コスト 21)
        // 合計コスト = 13 + 21 = 34
        let boards = vec![8, 5, 8];
        assert_eq!(resolve(&boards), 34);
    }

    #[test]
    fn test_example2() {
        // 例: [1, 2, 3, 4, 5]
        // 最適な結合方法の一例:
        // 1 + 2 = 3   (コスト 3)
        // 3 + 3 = 6   (コスト 6)
        // 4 + 5 = 9   (コスト 9)
        // 6 + 9 = 15  (コスト 15)
        // 合計コスト = 3 + 6 + 9 + 15 = 33
        let boards = vec![1, 2, 3, 4, 5];
        assert_eq!(resolve(&boards), 33);
    }

    #[test]
    fn test_equal_boards() {
        // 例: [5, 5, 5, 5]
        // 結合例:
        // 5 + 5 = 10   (コスト 10)
        // 5 + 5 = 10   (コスト 10)
        // 10 + 10 = 20 (コスト 20)
        // 合計コスト = 10 + 10 + 20 = 40
        let boards = vec![5, 5, 5, 5];
        assert_eq!(resolve(&boards), 40);
    }
}
