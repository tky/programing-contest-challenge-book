use itertools::Itertools;

fn main() {
    check(4, 10, &vec![1, 3, 5, 7]);
    check(3, 10, &vec![1, 3, 5]);
}

fn check(n: u32, m: u32, k: &Vec<u32>) {
    println!("n={}, m={}, k={:?}", n, m, k);
    let sum = k.iter().sum();
    if m > sum {
        println!("No(和が{}になるような出力は存在しません", m);
        return
    }
    if m == sum {
        println!("Yes(たとえば{:?}のようにでればば和が{}になります)", k, m);
        return
    }
    for i in 1..n {
        for ts in k.iter().combinations(i.try_into().unwrap()) {
            let s = ts.iter().fold(0, |acc, &x| acc + x);
            if m == s {
                println!("Yes(たとえば{:?}のようにでればば和が{}になります)",ts, m);
                return
            }
        }
    }
    println!("No(和が{}になるような出力は存在しません", m);
}
