fn main() {
    solve(9);
}

// Carmichael Numbers
// 任意の1 < x < nに対して x^n = x mod nが成り立つような素数
//
// 任意のxなのでx = 2としてよい
// n = 9の場合
// 2^9 mod 9 = (2^8 mod 9) * (2^1 mod 9) mod 9
// = (4 * 2) mod 9 = 8
// != 2 mod 9
fn solve(n: usize) -> bool {
    if is_prime(n) {
        return false;
    }
    let mut res = 1;
    let md = n;
    let mut x = 2;
    let mut n = n;

    while n > 0 {
        println!(
            "x: {}, n: {}, res: {}, flag: {}: x % md: {}",
            x,
            n,
            res,
            (n & 1) > 0,
            x % md
        );
        if (n & 1) > 0 {
            res = res * x % md
        }
        x = x * x % md;

        n >>= 1;
    }
    res % md == 2 % md
}

fn is_prime(n: usize) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ if n % 2 == 0 => false,
        _ => {
            let s = n.isqrt() as usize;
            (3..=s).step_by(2).all(|i| n % i != 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(solve(4), false);
        // 素数
        // assert_eq!(solve(5), false);
        assert_eq!(solve(561), true);
        assert_eq!(solve(562), false);
    }
}
