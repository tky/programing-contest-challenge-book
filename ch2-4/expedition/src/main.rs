use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn resolve(l: i32, p: i32, a: &[i32], b: &[i32]) -> i32 {
    let mut ans = 0;
    let mut pos = 0;
    let mut tank = p;
    let mut que: BinaryHeap<i32> = BinaryHeap::new();

    // 最後の位置をゴールに設定
    // ゴール地点を給油なしの補給所として追加
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.push(l);
    b.push(0);

    for i in 0..a.len() {
        let d = (a[i] - pos) as i32;
        while tank < d {
            if que.is_empty() {
                return -1;
            }
            tank += que.pop().unwrap();
            ans += 1;
        }
        tank -= d;
        pos = a[i];
        que.push(b[i]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let l = 25;
        let p = 10;
        let a = vec![10, 14, 20, 21];
        let b = vec![10, 5, 2, 4];
        assert_eq!(resolve(l, p, &a, &b), 2);
    }
}
