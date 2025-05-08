fn main() {
    println!("Hello, world!");
    segment_sieve(0, 100);
}

fn segment_sieve(a: usize, b: usize) -> usize {
    let sqrtb = (b as f64).sqrt() as usize;
    let mut small_primes = vec![true; sqrtb + 1];
    let mut large_primes = vec![true; b];
    small_primes[0] = false;
    small_primes[1] = false;
    large_primes[0] = false;
    large_primes[1] = false;
    large_primes[0..a].fill(false);

    for i in 2..sqrtb + 1 {
        if small_primes[i] {
            for j in (i * i..=sqrtb).step_by(i) {
                small_primes[j] = false;
            }
            for j in (i * i..b).step_by(i) {
                large_primes[j] = false;
            }
        }
    }
    large_primes.iter().filter(|&p| *p).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_sieve() {
        assert_eq!(segment_sieve(0, 10), 4);
        assert_eq!(segment_sieve(22, 37), 3);
    }
}
