struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

// 1. 始点に0を書き込む
// 2. 未確定の地点から最も小さい値を持つ地点を１つ選びその値を確定させる
// 3. 2で確定した地点から直接繋がっていて、かつ未確定な地点に対しかかる時間を計算し、書き込まれる値より小さければ更新する
// 4. 全ての地点が確定していれば終了
// 5. 全ての地点が確定していれば終了。そうでなければ2に戻る
fn resolve(n: usize, edges: &[Edge]) -> usize {
    let mut distances = vec![usize::MAX; n];
    let mut confirmeds = vec![false; n];

    distances[0] = 0;

    while let Some((index, &value)) = distances
        .iter()
        .enumerate()
        .filter(|(i, _)| !confirmeds[*i])
        .min_by_key(|&(_, &v)| v)
    {
        if value == usize::MAX {
            break;
        }
        confirmeds[index] = true;
        let value = distances[index];

        for e in edges {
            // from -> to
            if e.from == index && !confirmeds[e.to] {
                let cost = value + e.cost;
                if cost < distances[e.to] {
                    distances[e.to] = cost;
                }
            }
            // to -> from
            if e.to == index && !confirmeds[e.from] {
                let cost = value + e.cost;
                if cost < distances[e.from] {
                    distances[e.from] = cost;
                }
            }
        }
    }
    distances[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let edges = vec![
            Edge {
                from: 0,
                to: 1,
                cost: 2,
            },
            Edge {
                from: 0,
                to: 2,
                cost: 5,
            },
            Edge {
                from: 2,
                to: 1,
                cost: 4,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 2,
            },
            Edge {
                from: 1,
                to: 3,
                cost: 6,
            },
            Edge {
                from: 1,
                to: 4,
                cost: 10,
            },
            Edge {
                from: 3,
                to: 5,
                cost: 1,
            },
            Edge {
                from: 4,
                to: 5,
                cost: 3,
            },
            Edge {
                from: 4,
                to: 6,
                cost: 5,
            },
            Edge {
                from: 5,
                to: 6,
                cost: 9,
            },
        ];
        assert_eq!(resolve(7, &edges), 16);
    }
}
