use itertools::{izip, Itertools};

#[derive(Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT
}

struct Ant {
    point: i32,
    direction: Direction
}

fn main() {
    let (min, max) = solve(10, vec![2, 6, 7]);
    println!("min: {}, max: {}", min, max);
}

fn solve(l: i32, positions: Vec<i32>) -> (i32, i32) {
    let length = positions.len();

    let mut min = l * length as i32;
    let mut max = 0;

    let patterns =
        (0..length)
        .map(|_| &[Direction::LEFT, Direction::RIGHT])
        .multi_cartesian_product();

    for pattern in patterns {
        let ants = 
            izip!(positions.iter(), pattern.iter().cloned())
            .map(|(p, d)| Ant { point: *p, direction: *d });
        let distance = calculate(l, ants.collect());
        if distance < min {
            min = distance;
        }
        if distance > max {
            max = distance;
        }
    }
    (min, max)
}


// 指定した状態での移動距離を計算する
// ありがぶつかると方向転換するがお互いが反転するため方向転換は無視できる
//  
// 左を基点として考える
// 向きが左だったら現在の位置が端までの距離
// 向きが右だったら棒の長さから現在の位置を引いた距離
fn calculate(length: i32, ants: Vec<Ant>) -> i32 {
    ants.iter().map(|ant| {
        match ant.direction {
            Direction::LEFT => ant.point,
            Direction::RIGHT => length - ant.point
        }}).max().unwrap()
}
