use queues::*;

fn main() {
    let map = to_vec_2d(&[
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
    let result = solve(map, 10, 10);
    print_result(result);
}

type Maze = Vec<Vec<char>>;

static INF: i32 = 1000000000;
static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_start(map: &Maze, n: usize, m: usize) -> (usize, usize) {
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("Start point not found");
}

fn solve(map: Maze, n: usize, m: usize) -> Vec<Vec<i32>> {
    let (sx, sy) = find_start(&map, n, m);
    let mut distance = vec![vec![INF; m]; n];
    let mut q: Queue<(usize, usize)> = queue![];

    q.add((sx, sy)).unwrap();
    distance[sx][sy] = 0;

    while q.size() != 0 {
        let (x, y) = q.remove().unwrap();
        if map[x][y] == 'G' {
            break;
        }
        for &(dx, dy) in &DIRECTIONS {
            if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                if nx < n && ny < m {
                    if map[nx][ny] != '#' && distance[nx][ny] == INF {
                        q.add((nx, ny)).unwrap();
                        distance[nx][ny] = distance[x][y] + 1;
                    }
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

fn to_vec_2d(lines: &[&str]) -> Maze {
    lines.iter().map(|line| line.chars().collect()).collect()
}
