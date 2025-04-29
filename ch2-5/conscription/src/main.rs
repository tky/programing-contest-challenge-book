use std::cmp::Reverse;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

#[derive(Debug, Clone, Copy)]
struct Relation {
    boy: usize,
    girl: usize,
    intimacy: usize,
}

// 親密度のグラフが複数の森に分割されている可能性があるためPrim法では解決できない
fn resolve(boys_count: usize, girls_count: usize, relations: &mut [Relation]) -> usize {
    let mut uf = QuickUnionUf::<UnionBySize>::new(boys_count + girls_count);
    relations.sort_by_key(|r| Reverse(r.intimacy));

    let mut total_intimacy = 0;
    for Relation {
        boy,
        girl,
        intimacy,
    } in relations.iter().copied()
    {
        println!("boy: {}, girl: {}, intimacy: {}", boy, girl, intimacy);
        if uf.find(boy + girls_count) != uf.find(girl) {
            uf.union(boy + girls_count, girl);
            total_intimacy += intimacy;
        }
    }
    10000 * (boys_count + girls_count) - total_intimacy
}

fn main() {
    let mut relations = vec![
        Relation {
            boy: 0,
            girl: 0,
            intimacy: 500,
        },
        Relation {
            boy: 1,
            girl: 0,
            intimacy: 100,
        },
    ];
    assert_eq!(resolve(2, 2, &mut relations), 40000 - 600);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let mut relations = vec![
            Relation {
                boy: 4,
                girl: 3,
                intimacy: 6831,
            },
            Relation {
                boy: 1,
                girl: 3,
                intimacy: 4583,
            },
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 6592,
            },
            Relation {
                boy: 1,
                girl: 0,
                intimacy: 3063,
            },
            Relation {
                boy: 3,
                girl: 3,
                intimacy: 4975,
            },
            Relation {
                boy: 1,
                girl: 3,
                intimacy: 2049,
            },
            Relation {
                boy: 4,
                girl: 2,
                intimacy: 2104,
            },
            Relation {
                boy: 2,
                girl: 2,
                intimacy: 781,
            },
        ];
        assert_eq!(resolve(5, 5, &mut relations), 71071);
    }

    #[test]
    fn test_no_relations() {
        let mut relations = vec![];
        assert_eq!(resolve(3, 4, &mut relations), 10000 * (3 + 4));
    }

    #[test]
    fn test_zero_intimacy() {
        let mut relations = vec![
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 0,
            },
            Relation {
                boy: 1,
                girl: 1,
                intimacy: 0,
            },
        ];
        assert_eq!(resolve(2, 2, &mut relations), 10000 * 4);
    }

    #[test]
    fn test_all_connected_high_discount() {
        let mut relations = vec![
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 9999,
            },
            Relation {
                boy: 1,
                girl: 1,
                intimacy: 9999,
            },
            Relation {
                boy: 2,
                girl: 2,
                intimacy: 9999,
            },
        ];
        assert_eq!(resolve(3, 3, &mut relations), 10000 * 6 - (9999 * 3));
    }

    #[test]
    fn test_duplicate_edges_only_best_used() {
        let mut relations = vec![
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 500,
            },
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 700,
            }, // better intimacy, should override
        ];
        assert_eq!(resolve(1, 1, &mut relations), 10000 * 2 - 700);
    }

    // グラフが複数の森に分かれているためPrim法では解けない
    #[test]
    fn test_split_forest_case() {
        let mut relations = vec![
            Relation {
                boy: 0,
                girl: 0,
                intimacy: 5000,
            }, // B0-G0
            Relation {
                boy: 1,
                girl: 1,
                intimacy: 6000,
            }, // B1-G1
            Relation {
                boy: 3,
                girl: 3,
                intimacy: 7000,
            }, // B3-G3
            Relation {
                boy: 4,
                girl: 4,
                intimacy: 8000,
            }, // B4-G4
               // B2-G2 は親密度なし → 孤立
        ];

        // 男5人、女5人 → 基本コスト 100000
        // 最大割引合計 = 5000 + 6000 + 7000 + 8000 = 26000
        // → 最小徴兵コスト = 100000 - 26000 = 74000
        assert_eq!(resolve(5, 5, &mut relations), 74000);
    }
}
