use std::vec;

#[derive(Debug, Clone, Copy)]
struct FriendlyConstraint {
    cow_a: usize,
    cow_b: usize,
    max_distance: usize,
}

#[derive(Debug, Clone, Copy)]
struct HostileConstraint {
    cow_a: usize,
    cow_b: usize,
    min_distance: usize,
}

fn solve(cow_count: usize, friends: &[FriendlyConstraint], hostiles: &[HostileConstraint]) -> i32 {
    let mut positions = vec![usize::MAX; cow_count];
    positions[0] = 0;
    let mut updated = true;
    for _ in 0..(cow_count - 1) {
        // 更新がない場合に早期終了
        if !updated {
            break;
        }
        updated = false;
        for i in 0..(cow_count - 1) {
            if positions[i + 1] < usize::MAX && positions[i] > positions[i + 1] {
                positions[i] = positions[i + 1];
                updated = true;
            }
            for FriendlyConstraint {
                cow_a,
                cow_b,
                max_distance,
            } in friends.iter().copied()
            {
                if positions[cow_a] < usize::MAX
                    && positions[cow_a] + max_distance < positions[cow_b]
                {
                    positions[cow_b] = positions[cow_a] + max_distance;
                    updated = true;
                }
            }

            for HostileConstraint {
                cow_a,
                cow_b,
                min_distance,
            } in hostiles.iter().copied()
            {
                if positions[cow_b] < usize::MAX
                    && positions[cow_a] > positions[cow_b] - min_distance
                {
                    positions[cow_a] = positions[cow_b] - min_distance;
                    updated = true;
                }
            }
        }
    }
    if positions[cow_count - 1] == usize::MAX {
        return -2;
    }
    // 最後にもう一度ループして更新できた場合は負閉路があるので-1を返す
    for i in 0..(cow_count - 1) {
        if positions[i + 1] < usize::MAX && positions[i] > positions[i + 1] {
            return -1;
        }
    }
    for &FriendlyConstraint {
        cow_a,
        cow_b,
        max_distance,
    } in friends.iter()
    {
        if positions[cow_a] < usize::MAX && positions[cow_a] + max_distance < positions[cow_b] {
            return -1;
        }
    }
    for &HostileConstraint {
        cow_a,
        cow_b,
        min_distance,
    } in hostiles.iter()
    {
        if positions[cow_b] < usize::MAX && positions[cow_a] > positions[cow_b] - min_distance {
            return -1;
        }
    }
    positions[cow_count - 1] as i32
}

fn main() {
    println!("Hello, world!");
    let friends = vec![
        FriendlyConstraint {
            cow_a: 0,
            cow_b: 2,
            max_distance: 10,
        },
        FriendlyConstraint {
            cow_a: 1,
            cow_b: 3,
            max_distance: 20,
        },
    ];
    let hostiles = vec![HostileConstraint {
        cow_a: 1,
        cow_b: 2,
        min_distance: 3,
    }];

    solve(4, &friends, &hostiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let friends = vec![
            FriendlyConstraint {
                cow_a: 0,
                cow_b: 2,
                max_distance: 10,
            },
            FriendlyConstraint {
                cow_a: 1,
                cow_b: 3,
                max_distance: 20,
            },
        ];
        let hostiles = vec![HostileConstraint {
            cow_a: 1,
            cow_b: 2,
            min_distance: 3,
        }];

        assert_eq!(solve(4, &friends, &hostiles), 27);
    }
    #[test]
    fn test_balanced_constraints() {
        let friends = vec![FriendlyConstraint {
            cow_a: 0,
            cow_b: 1,
            max_distance: 5,
        }];
        let hostiles = vec![HostileConstraint {
            cow_a: 0,
            cow_b: 1,
            min_distance: 5,
        }];
        assert_eq!(solve(2, &friends, &hostiles), 5);
    }

    #[test]
    fn test_unreachable_last_cow() {
        let friends = vec![FriendlyConstraint {
            cow_a: 0,
            cow_b: 1,
            max_distance: 5,
        }];
        let hostiles = vec![];
        assert_eq!(solve(4, &friends, &hostiles), -2);
    }

    // いくらでも離れることが可能な場合は-2を返す
    // 先頭に制約がないので1頭目と2頭目の距離はいくらでも離すことができてしまう
    #[test]
    fn test_solve2() {
        let friends = vec![FriendlyConstraint {
            cow_a: 1,
            cow_b: 3,
            max_distance: 20,
        }];
        let hostiles = vec![HostileConstraint {
            cow_a: 2,
            cow_b: 3,
            min_distance: 3,
        }];

        assert_eq!(solve(4, &friends, &hostiles), -2);
    }
}
