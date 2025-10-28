use itertools::Itertools;

fn main() {
    solve1(3, 10, &vec![1, 3, 5]);
    solve2(3, 10, &vec![1, 3, 5]);
    solve3(3, 10, &vec![1, 3, 5]);
}

// kから4回選んで和がmになるかの判定を行う
//
// ka + kb + kc + kd = m であれば4回選んだ和がmになったことになる
// この解放前パターンをチェックしており、計算量はO(n^4)である
fn solve1(n: u32, m: u32, k: &Vec<u32>) {
    println!("n: {}, m: {}, k: {:?}", n, m, k);
    let mut f: bool = false;
    for a in 0..n as usize {
        for b in 0..n as usize {
            for c in 0..n as usize {
                for d in 0..n as usize {
                    if k[a] + k[b] + k[c] + k[d] == m {
                        f = true;
                    }
                }
            }
        }
    }

    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 改良版
// ka + kb + kc + kd = m
// より、kd = m - (ka + kb + kc) と変形できる
// つまり配列kにkdが存在するかを二分探索で調べればよい
// 配列の探索は２分探索を用いることで計算量はO(n^3 log n)に削減できる
// kは事前にソートしておく必要がある
fn solve2(n: u32, m: u32, k: &Vec<u32>) {
    println!("n: {}, m: {}, k: {:?}", n, m, k);
    let mut f: bool = false;
    for a in 0..n as usize {
        for b in 0..n as usize {
            for c in 0..n as usize {
                // rustのu32はマイナスの値を取れないので事前にチェック
                let sum = k[a] + k[b] + k[c];
                if m >= sum {
                    if binary_search(m - sum, k) {
                        f = true;
                    }
                }
            }
        }
    }

    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}

// さらに改良版
// ka + kb + kc + kd = m
// より、(ka + kb) + (kc + kd) = m と変形できる
// (ka + kb)の組み合わせを事前に計算しておき、(kc + kd) = m - (ka + kb)が存在するかを二分探索で調べればよい
// (ka + kb)の組み合わせはn^2通りあるので、事前計算にO(n^2)かかる
// 計算量はO(n^2 log n)に削減できる
fn solve3(n: u32, m: u32, k: &Vec<u32>) {
    println!("n: {}, m: {}, k: {:?}", n, m, k);

    // (ka + kb)の組み合わせを事前計算してsortしておく
    let mut kk = k
        .iter()
        .combinations(2)
        .map(|vs| vs.iter().copied().sum::<u32>())
        .collect_vec();
    kk.sort();

    let mut f = false;
    // (kc + kd) = m - (ka + kb)が存在するかを二分探索で調べる
    for a in 0..n as usize {
        for b in 0..n as usize {
            let sum = k[a] + k[b];
            if m >= sum {
                if binary_search(m - sum, &kk) {
                    f = true;
                }
            }
        }
    }
    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn binary_search(x: u32, k: &Vec<u32>) -> bool {
    let mut l = 0;
    let mut r = k.len();
    while r - l >= 1 {
        let i = (l + r) / 2;
        if k[i] == x {
            return true;
        } else if k[i] < x {
            l = i + 1;
        } else {
            r = i;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    #[test]
    fn test_binary_search_with_odd_input() {
        let k = vec![1, 3, 5];
        assert_eq!(true, binary_search(1, &k));
        assert_eq!(true, binary_search(3, &k));
        assert_eq!(true, binary_search(5, &k));
        assert_eq!(false, binary_search(2, &k));
        assert_eq!(false, binary_search(4, &k));
        assert_eq!(false, binary_search(6, &k));
    }

    #[test]
    fn test_binary_search_with_even_input() {
        let k = vec![1, 3, 5, 7];
        assert_eq!(true, binary_search(1, &k));
        assert_eq!(true, binary_search(3, &k));
        assert_eq!(true, binary_search(5, &k));
        assert_eq!(true, binary_search(7, &k));
        assert_eq!(false, binary_search(2, &k));
        assert_eq!(false, binary_search(4, &k));
        assert_eq!(false, binary_search(6, &k));
        assert_eq!(false, binary_search(8, &k));
    }
}
