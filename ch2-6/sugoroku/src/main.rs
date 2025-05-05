fn main() {
    println!("Hello, world!");
    solve(4, 11);
}

fn solve(a: i32, b: i32) -> Option<(usize, usize, usize, usize)> {
    let (x, y, d) = extgcd(a, b);
    if d != 1 {
        return None;
    }
    let p = x.max(0) as usize;
    let q = (-x).max(0) as usize;
    let r = y.max(0) as usize;
    let s = (-y).max(0) as usize;
    Some((p, q, r, s))
}

// let (x, y, d) = extgcd(u, v);
// ux + vy = d
fn extgcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (x1, y1, d) = extgcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (x, y, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extgcd() {
        let (x, y, d) = extgcd(4, 11);
        assert_eq!(x * 4 + y * 11, d);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_solve() {
        let result = solve(4, 11);
        assert_eq!(result, Some((3, 0, 0, 1)));
    }
    #[test]
    fn test_solve_non_coprime() {
        assert_eq!(solve(6, 4), None);
    }
    #[test]
    fn test_solve_7_3() {
        assert_eq!(solve(7, 3), Some((1, 0, 0, 2)));
    }

    #[test]
    fn test_solve_2_5() {
        assert_eq!(solve(2, 5), Some((0, 2, 1, 0)));
    }
}
