use itertools::Itertools;

fn main() {
    resolve(&vec![0, 1, 2, 3, 4, 5], 8);
    resolve(&vec![0, 1, 2, 3, 4, 5], 17);

    resolve2(&vec![0, 1, 2, 3, 4, 5], 8);
    resolve2(&vec![0, 1, 2, 3, 4, 5], 17);
}

// 全ての組み合わせを試す
fn resolve(a: &Vec<u32>, k: u32) {
    for i in 1..a.len() {
        if let Some(c) = a.iter().combinations(i).find(|c| {
            let sum = c.iter().fold(0, |acc, &x| acc + x);
            sum == k
        }) {
            print_success(&c, k);
            return;
        }
    }
    println!("No");
}

fn resolve2(a: &Vec<u32>, k: u32) {
    if dfs(a, 0, 0, k) {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 深さy優先探索で部分和問題を解く
// iまででsumを作り、残りi以降でkを作れるか判定する
// 最初は dfs(a, 0, 0, k) と呼び出す必要がある
// dfs(0, 0)
// ├─ dfs(1, 0)  ← 1を使わない
// │  ├─ dfs(2, 0)  ← 2を使わない
// │  │  ├─ dfs(3, 0)  ← 3を使わない → false
// │  │  └─ dfs(3, 3)  ← 3を使う → true ✓
// │  └─ ...
// └─ ...
fn dfs(a: &Vec<u32>, i: usize, sum: u32, k: u32) -> bool {
    if a.len() == i {
        return sum == k;
    }
    // a[i]を使わない場合
    // a[i+1]以降でsumを作れるか判定する
    if dfs(a, i + 1, sum, k) {
        return true;
    }
    // a[i]を使う場合
    // a[i+1]以降でsum + a[i]を作れるか判定する
    if dfs(a, i + 1, sum + a[i], k) {
        return true;
    }
    false
}

fn print_success(a: &Vec<&u32>, k: u32) {
    let result = a.iter().join(" + ");
    println!("Yes({} = {})", k, result);
}
