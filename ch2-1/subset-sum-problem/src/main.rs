use itertools::Itertools;

fn main() {
    resolve(&vec![0, 1, 2, 3, 4, 5], 8);
    resolve(&vec![0, 1, 2, 3, 4, 5], 17);

    resolve2(&vec![0, 1, 2, 3, 4, 5], 8);
    resolve2(&vec![0, 1, 2, 3, 4, 5], 17);
}

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

fn dfs(a: &Vec<u32>, i: usize, sum: u32, k: u32) -> bool {
    if a.len() == i {
        return sum == k
    }
    if dfs(a, i + 1, sum, k) {
        return true
    }
    if dfs(a, i + 1, sum + a[i], k) {
        return true
    }
    false
}

fn print_success(a: &Vec<&u32>, k: u32) {
    let result = a.iter().join(" + ");
    println!("Yes({} = {})", k, result);
}
