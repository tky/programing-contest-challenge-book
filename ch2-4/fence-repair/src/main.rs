use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn solve(l: &[usize]) -> usize {
    let mut heap: BinaryHeap<Reverse<usize>> = l.iter().map(|&x| Reverse(x)).collect();
    let mut ans = 0;

    while heap.len() > 1 {
        let Reverse(a) = heap.pop().unwrap();
        let Reverse(b) = heap.pop().unwrap();
        ans += a + b;
        heap.push(Reverse(a + b));
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(&[8, 5, 8]), 34);
    }
}
