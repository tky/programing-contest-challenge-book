fn main() {
    let map_str = [
        "1 0 0 2 1 0",
        "1 1 0 0 0 0",
        "0 0 0 0 0 3",
        "0 0 0 0 0 0",
        "1 0 0 0 0 1",
        "0 1 1 1 1 1",
    ];

    let mut map = map_str
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u16>>>();
    let result = solve(&mut map);
    println!("{}", result);
}

type MapRef<'a> = &'a [Vec<u16>];

enum SlideResult {
    Goal,
    // 止まった位置、障害物の位置
    HitObstacle((usize, usize), (usize, usize)),
    OutOfBounds,
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
    (-1, 0), // Up
];

fn solve(map: MapRef) -> isize {
    let start = find_position(map, 2).expect("Start position not found");

    dfs(map, start, map.len(), map[0].len(), 0, vec![])
}

fn dfs(
    map: MapRef,
    start: (usize, usize),
    w: usize,
    h: usize,
    trial: usize,
    obstacles: Vec<(usize, usize)>,
) -> isize {
    if trial > 10 {
        return -1;
    }
    let mut best: isize = -1;
    for direction in DIRECTIONS.iter() {
        match curling(map, start, *direction, w, h, &obstacles) {
            SlideResult::Goal => {
                let moves = (trial + 1) as isize;
                best = if best == -1 { moves } else { best.min(moves) };
            }
            SlideResult::HitObstacle(stop, obstacle) => {
                let mut new_vec = obstacles.clone();
                new_vec.push(obstacle);
                let res = dfs(map, stop, w, h, trial + 1, new_vec);
                if res != -1 {
                    best = if best == -1 { res } else { best.min(res) };
                }
            }
            SlideResult::OutOfBounds => continue,
        }
    }
    best
}

// 指定した方向へ石を滑らせる
fn curling(
    map: MapRef,
    start: (usize, usize),
    direction: (isize, isize),
    w: usize,
    h: usize,
    // 既に取り除いた障害物
    // mapを更新するのは大変なので取り除いた障害物の位置を保持する
    obstacles: &Vec<(usize, usize)>,
) -> SlideResult {
    let mut x = start.0 as isize;
    let mut y = start.1 as isize;
    loop {
        let next_x = x + direction.0;
        let next_y = y + direction.1;
        if next_x < 0 || next_x >= h as isize || next_y < 0 || next_y >= w as isize {
            return SlideResult::OutOfBounds;
        }
        match map[next_x as usize][next_y as usize] {
            1 => {
                if obstacles.contains(&(next_x as usize, next_y as usize)) {
                    x = next_x;
                    y = next_y;
                } else {
                    return SlideResult::HitObstacle(
                        (x as usize, y as usize),
                        (next_x as usize, next_y as usize),
                    );
                }
            }
            3 => return SlideResult::Goal,
            _ => {
                x = next_x;
                y = next_y;
            }
        }
    }
}

fn find_position(map: MapRef, v: u16) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == v {
                return Some((i, j));
            }
        }
    }
    return None;
}
