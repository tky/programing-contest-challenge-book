use itertools::Itertools;

fn main() {
    solve1(3, 10, &vec![1, 3, 5]);
    solve2(3, 10, &vec![1, 3, 5]);
    solve3(3, 10, &vec![1, 3, 5]);
}

fn solve1(n: u32, m: u32, k: &Vec<u32>) {
    println!("n: {}, m: {}, k: {:?}", n, m, k);
    let mut f: bool = false;
    for a in 0..n as usize {
        for b in 0..n as usize {
            for c in 0..n as usize {
                for d in 0..n as usize {
                    // ka + kb + kc + kd = m
                    // => 
                    // kd = m - ka - kb - kc
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

fn solve3(n: u32, m: u32, k: &Vec<u32>) {
    println!("n: {}, m: {}, k: {:?}", n, m, k);
    let mut kk = k.iter().combinations(2).map(|v| {
        v.iter().fold(0, |acc, &x| acc + x)
    }).collect_vec();
    kk.sort();

    let mut f = false;
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
            return true
        } else if k[i] < x {
            l = i + 1;
        } else {
            r = i;
        }
    }
    return false
}

#[cfg(test)]
mod tests {
    use super::{binary_search};
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
