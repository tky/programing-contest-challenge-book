fn main() {
    println!("Hello, world!");
    eratosthenes(10);
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    let limit = n.isqrt() + 1;
    for i in 2..limit {
        if primes[i] {
            for j in (i * i..=n).step_by(i) {
                primes[j] = false;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eratosthenes() {
        let n = 30;
        let primes = eratosthenes(n);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
