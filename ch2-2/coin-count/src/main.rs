use std::cmp;

fn main() {
    let (c1, c5, c10, c50, c100, c500, a) = (3, 2, 1, 3, 0, 2, 620);
    println!("{}", solve(c1, c5, c10, c50, c100, c500, a));
}

fn solve(c1: usize, c5: usize, c10: usize, c50: usize, c100: usize, c500: usize, a: usize) -> usize {
    let mut ans = 0;
    let mut remain = a;

    for (numer_of_coin, price) in [(c500, 500), (c100, 100), (c50, 50), (c10, 10), (c5, 5), (c1, 1)] {
        if remain == 0 {
            break;
        }
        if remain >= price {
            let paid_count = cmp::min(numer_of_coin, remain / price);
            let paid_amount = paid_count * price;

            ans += paid_count;
            remain -= paid_amount;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::{solve};

    #[test]
    fn test_solve() {
        let (c1, c5, c10, c50, c100, c500, a) = (3, 2, 1, 3, 0, 2, 620);
        assert_eq!(solve(c1, c5, c10, c50, c100, c500, a), 6);
    }
}
