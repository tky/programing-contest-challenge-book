use itertools::{izip, Itertools};

#[derive(Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT
}

// Antの構造体の大輔タイム'aについて
//
// この構造体が有効である間はdirectionのライフタイムも有効である必要がある
// direction（ライフタイム 'a'）: |--------------------|
// Ant（ライフタイム 'a'）      :     |----------|
//
//　Antの大輔タイムは
//  構造体 Ant が生きている間、その direction 参照先も生きている必要があります。これはライフタイム 'a によって保証されます。
//  ライフタイム 'a は、Ant のライフタイムが direction のライフタイムに依存していることを示します。
//  Ant のライフタイム ≤ direction のライフタイム という関係が成り立ちます。
struct Ant<'a> {
    point: i32,
    direction: &'a Direction,
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
            izip!(positions.iter(), pattern.iter())
            .map(|(p, d)| Ant { point: *p, direction: *d })
            .collect::<Vec<_>>();
        let distance = calculate(l, &ants);
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
fn calculate(length: i32, ants: &[Ant]) -> i32 {
    ants.iter().map(|ant| {
        match ant.direction {
            Direction::LEFT => ant.point,
            Direction::RIGHT => length - ant.point
        }}).max().unwrap()
}
