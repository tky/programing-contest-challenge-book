fn main() {
    let map_str = [
        "....#.", ".....#", "......", "......", "......", "......", "......", "#@...#", ".#..#.",
    ];

    let map: Vec<Vec<char>> = map_str.iter().map(|line| line.chars().collect()).collect();
    let ans = solve(&map);
    println!("The answer is: {}", ans);
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];
type MapRef<'a> = &'a [Vec<char>];
type VisitMap = Vec<Vec<bool>>;
type VisitMapMut<'a> = &'a mut VisitMap;

fn solve(map: MapRef) -> usize {
    let pos = find_position(&map, '@');
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let ans = calculate(
        map,
        &mut visited,
        map[0].len() as isize,
        map.len() as isize,
        pos,
    );

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited[i][j] {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!("");
    }
    ans
}

fn calculate(map: MapRef, visited: VisitMapMut, w: isize, h: isize, pos: (usize, usize)) -> usize {
    let (x, y) = pos;
    if map[x][y] == '#' {
        return 0;
    }
    let mut ans = 0;
    if !visited[x][y] {
        ans += 1;
        visited[x][y] = true;
    }
    for (i, j) in DIRECTIONS {
        let next_x = x as isize + i;
        let next_y = y as isize + j;
        if next_x >= 0 && next_x < h && next_y >= 0 && next_y < w {
            if visited[next_x as usize][next_y as usize] {
                continue;
            }
            ans += calculate(map, visited, w, h, (next_x as usize, next_y as usize));
        }
    }
    ans
}

fn find_position(map: MapRef, c: char) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == c {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let map_str = [
            "....#.", ".....#", "......", "......", "......", "......", "......", "#@...#",
            ".#..#.",
        ];

        let map: Vec<Vec<char>> = map_str.iter().map(|line| line.chars().collect()).collect();
        let ans = solve(&map);
        assert_eq!(ans, 45);
    }

    #[test]
    fn test_solve2() {
        let map_str = [
            "..#..#..#..",
            "..#..#..#..",
            "..#..#..###",
            "..#..#..#@.",
            "..#..#..#..",
            "..#..#..#..",
        ];

        let map: Vec<Vec<char>> = map_str.iter().map(|line| line.chars().collect()).collect();
        let ans = solve(&map);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_solve3() {
        let map_str = [
            "..#.#..", "..#.#..", "###.###", "...@...", "###.###", "..#.#..", "..#.#..",
        ];

        let map: Vec<Vec<char>> = map_str.iter().map(|line| line.chars().collect()).collect();

        let ans = solve(&map);
        assert_eq!(ans, 13);
    }
}
