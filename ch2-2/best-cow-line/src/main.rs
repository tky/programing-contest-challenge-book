fn main() {
    let vs = "ACDBCB".chars().collect::<Vec<char>>();
    let result = resolve(&vs);
    println!("{}", result);
}

fn resolve(vs: &[char]) -> String {
    let mut result = String::new();
    let mut s = 0;
    let mut e = vs.len() - 1;
    while s <= e {
        println!("s: {}, e: {}, {:?} {}", s, e, &vs[s..=e], result);
        // 先頭の方が早い場合は先頭を先頭を追加
        if vs[s] < vs[e] {
            result.push(vs[s]);
            s += 1;
        // 末尾の方が早い場合は末尾を追加
        } else if vs[s] > vs[e] {
            result.push(vs[e]);
            e -= 1;
        } else {
            // 同じ場合は反転した文字列を比較
            let mut rev = vs[s..e].to_vec();
            rev.reverse();
            // 元の方が早ければ先頭を追加、そうでなければ末尾を追加
            if vs[s..e].to_vec() < rev {
                result.push(vs[s]);
                s += 1;
            } else {
                result.push(vs[e]);
                if e == 0 {
                    break;
                }
                e -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let test_cases = vec![
            ("ACDBCB", "ABCBCD"), // 基本ケース
            ("ABCDE", "ABCDE"),   // すでに昇順
            ("EDCBA", "ABCDE"),   // 逆順ケース
            ("AABBA", "AAABB"),   // 部分的に同じ文字
            ("ABABA", "AABAB"),   // 回文に近いケース
            ("ZZZZZ", "ZZZZZ"),   // 同じ文字
            ("A", "A"),           // 1文字ケース
            ("BA", "AB"),         // 2文字逆順ケース
            ("BBA", "ABB"),       // 3文字で同じ文字含み
            ("XYZZYX", "XXYYZZ"), // 対称なケース
        ];

        for (input, expected) in test_cases {
            let vs = input.chars().collect::<Vec<char>>();
            let result = resolve(&vs);
            assert_eq!(result, expected, "Failed on input: {}", input);
        }
    }
}
