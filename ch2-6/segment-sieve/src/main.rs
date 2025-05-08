fn main() {
    println!("Hello, world!");
    segment_sieve(0, 100);
}

fn segment_sieve(a: usize, b: usize) -> usize {
    assert!(a <= b);
    let limit = (b as f64).sqrt() as usize;

    let mut is_small = vec![true; limit + 1];
    is_small[0] = false;
    if limit >= 1 {
        is_small[1] = false;
    }
    for i in 2..=limit {
        if is_small[i] {
            for j in (i * i..limit).step_by(i) {
                is_small[j] = false;
            }
        }
    }

    let small_primes: Vec<usize> = (2..=limit).filter(|&i| is_small[i]).collect();

    let size = b - a + 1;
    let mut is_prime = vec![true; size];

    if a == 0 {
        is_prime[0] = false;
        if size > 1 {
            is_prime[1] = false;
        }
    } else if a == 1 {
        is_prime[0] = false;
    }

    for &p in &small_primes {
        let start = {
            let sq = p * p;
            if sq >= a {
                sq
            } else {
                let rem = a % p;
                // a = 10, p = 3の場合
                // rem = 10 % 3 = 1
                // 10..の中で最初に消すべき3の倍数は 10 + (3 - 1) = 12
                if rem == 0 { a } else { a + (p - rem) }
            }
        };
        for m in (start..=b).step_by(p) {
            is_prime[m - a] = false;
        }
    }

    (0..size).filter(|&i| is_prime[i]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_sieve() {
        assert_eq!(segment_sieve(0, 10), 4);
        assert_eq!(segment_sieve(23, 31), 3);
    }
}
