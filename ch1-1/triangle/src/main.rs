use itertools::Itertools;

fn main() {
    println!("{:?} {}", vec![2, 3, 4, 5, 10], solve(5, vec![2, 3, 4, 5, 10]));
}

fn solve(_n: u32, bars: Vec<u32>) -> u32 {
    let mut max = 0;
    for bs in bars.iter().combinations(3) {
        if bs[0] * bs[0] + bs[1] * bs[1] == bs[2] * bs[2] {
            let tmp = bs[0] + bs[1] + bs[2];
            if tmp > max {
                max = tmp;
            }
        }
    }
    max
}
