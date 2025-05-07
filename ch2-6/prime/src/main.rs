fn main() {
    is_prime(10);
}

// 一番シンプルな考え方だと２から(n-1)までの数で割り切れるか試せばよい
// ただし、実際はn^(1/2)までで十分である
//
//  nが合成数であればある整数aとbが存在して（両方とも1 < a < n)
// n = a * b
// となる。
// もし両方ともn^(1/2)より大きいすると
// a * b > n^(1/2) * n^(1/2) = n
// a * b > n となり矛盾が生じる
// したがって、nが合成数であればn^(1/2)より小さい整数a(or b)が存在する
// なので、nが合成数であればn^(1/2)までの整数で割り切れるか試せばよい
fn is_prime(n: usize) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        // 偶数は素数ではない
        _ if n % 2 == 0 => false,
        _ => {
            let s = n.isqrt() as usize;
            // 奇数だけ判定することでloopを半分にする
            (3..=s).step_by(2).all(|i| n % i != 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prive_zero() {
        assert_eq!(is_prime(0), false);
    }

    #[test]
    fn test_is_prive_one() {
        assert_eq!(is_prime(1), false);
    }
    #[test]
    fn test_is_prive_two() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);

        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(15), false);
    }
}
