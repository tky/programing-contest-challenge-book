use itertools::Itertools;

fn main() {
    resolve(&vec![0, 1, 2, 3, 4, 5], 8);
    resolve(&vec![0, 1, 2, 3, 4, 5], 17);
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

fn print_success(a: &Vec<&u32>, k: u32) {
    let result = a.iter().join(" + ");
    println!("Yes({} = {})", k, result);
}
