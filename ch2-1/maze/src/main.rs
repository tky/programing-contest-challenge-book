use queues::*;

fn main() {
  let map = 
    to_vec_2d(
    &[
    "#S######.#",
    "......#..#",
    ".#.##.##.#",
    ".#........",
    "##.##.####",
    "....#....#",
    ".#######.#",
    "....#.....",
    ".####.###.",
    "...#....G#",
  ]);
  let reslut = solve(map, 6, 10, 0, 1);
  print_result(reslut);
}

static INF: i32 = 1000000000;
static DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
];

fn solve(map: Vec<Vec<char>>, n: usize, m: usize, sx: usize, sy: usize) -> Vec<Vec<i32>> {
    let mut distance = vec![vec![INF; m]; n];
    let mut q: Queue<(usize, usize)> = queue![];

    let _ = q.add((sx, sy));
    distance[sx][sy] = 0;

    while q.size() != 0 {
        let p = q.remove().unwrap();
        if map[p.0][p.1] == 'G' {
            break;
        }
        for &(dx, dy) in &DIRECTIONS {
            let nx = p.0 as isize + dx;
            let ny = p.1 as isize + dy;
            // if inside the maze
            if 0 <= nx && nx < n as isize && 0 <= ny && ny < m as isize {
                // and next cell is not a wall and not visited
                let nx = nx as usize;
                let ny = ny as usize;
                if map[nx][ny] != '#' && distance[nx][ny] == INF {
                    let _ = q.add((nx, ny));
                    distance[nx][ny] = distance[p.0][p.1] + 1;
                }
            }
        }
    }
    distance
}

fn print_result(distance: Vec<Vec<i32>>) {
    for i in 0..distance.len() {
        for j in 0..distance[i].len() {
            if distance[i][j] == INF {
                print!("x ");
                continue;
            }
            print!("{} ", distance[i][j]);
        }
        println!();
    }
}

fn to_vec_2d(lines: &[&str]) -> Vec<Vec<char>> {
    lines.iter()
        .map(|line| line.chars().collect())
        .collect()
}
