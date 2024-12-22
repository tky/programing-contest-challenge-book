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
    // top
    if i > 0 && lake[i - 1][j] == 'W' {
        dfs(lake, i - 1, j, n, m);
    }
    // bottom
    if i + 1 < n  && lake[i + 1][j] == 'W' {
        dfs(lake, i + 1, j, n, m);
    }
    // left
    if j > 0 && lake[i][j - 1] == 'W' {
        dfs(lake, i, j - 1, n, m);
    }
    // right
    if j + 1 < m && lake[i][j + 1] == 'W' {
        dfs(lake, i, j + 1, n, m);
    }
    // top left
    if i > 0 && j > 0 && lake[i - 1][j - 1] == 'W' {
        dfs(lake, i - 1, j - 1, n, m);
    }
    // top right
    if i > 0 && j + 1 < m && lake[i - 1][j + 1] == 'W' {
        dfs(lake, i - 1, j + 1, n, m);
    }
    // bottom left
    if i + 1 < n && j > 0 && lake[i + 1][j - 1] == 'W' {
        dfs(lake, i + 1, j - 1, n, m);
    }
    // bottom right
    if i + 1 < n && j + 1 < m && lake[i + 1][j + 1] == 'W' {
        dfs(lake, i + 1, j + 1, n, m);
    }
}
