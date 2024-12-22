fn main() {
    // 10 * 12
    let mut lake =  vec! [
        vec!['W', 'W', '.', '.', '.', '.', '.', '.', '.', 'W', 'W', '.'],
        vec!['.', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', 'W'],
        vec!['.', '.', '.', 'W', 'W', 'W', '.', '.', '.', 'W', 'W', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'W', 'W', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'W', '.', '.'],
        vec!['.', '.', 'W', '.', '.', '.', '.', '.', '.', 'W', '.', '.'],
        vec!['.', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.'],
        vec!['W', 'W', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', '.'],
        vec!['.', 'W', 'W', 'W', '.', '.', '.', '.', '.', '.', 'W', '.'],
        vec!['.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', 'W', '.'],
    ];

    resolve(&mut lake, 10, 12);
}

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1),
];

fn resolve(lake: &mut [Vec<char>], n: usize, m: usize) {
    let mut count = 0;
    print_lake(lake, n, m);
    for i in 0..n {
        for j in 0..m {
            if lake[i][j] == 'W' {
                count += 1;
                dfs(lake, i, j, n, m);
            }
        }
    }
    println!("lake count: {}", count);
}

fn print_lake(lake: &mut [Vec<char>], n: usize, m: usize) {
    for i in 0..n {
        for j in 0..m {
            print!("{}", lake[i][j]);
        }
        println!();
    }
}

fn dfs(lake: &mut [Vec<char>], i: usize, j: usize, n: usize, m: usize) {
    if lake[i][j] == '.' {
        return;
    }
    lake[i][j] = '.';
    for &(dy, dx) in &DIRECTIONS {
        let ni = i as isize + dy;
        let nj = j as isize + dx;
        if 0 <= ni && ni < n as isize && 0 <= nj && nj < m as isize {
            dfs(lake, ni as usize, nj as usize, n, m);
        }
    }
}
