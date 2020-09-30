#![allow(dead_code)]

use bitset::Bitset;

fn atkin_sieve(n: usize) -> Vec<u64> {
    let mut sieve = Bitset::new(n);
    let lim = (n as f64).sqrt() as usize + 1;

    for z in (1..6).step_by(4) {
        for y in (z..lim).step_by(6) {
            for x in 1..lim {
                if 4 * x * x + y * y > n { break; }
                sieve.flip(4 * x * x + y * y);
            }
            for x in (y+1..lim).step_by(2) {
                if 3 * x * x - y * y > n { break; }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in (2..5).step_by(2) {
        for y in (z..lim).step_by(6) {
            for x in (1..lim).step_by(2) {
                if 3 * x * x + y * y > n { break; }
                sieve.flip(3 * x * x + y * y);
            }
            for x in (y+1..lim).step_by(2) {
                if 3 * x * x - y * y > n { break; }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in 1..3 {
        for y in (3..lim).step_by(6) {
            for x in (z..lim).step_by(3) {
                if 4 * x * x + y * y > n { break; }
                sieve.flip(4 * x * x + y * y);
            }
        }
    }

    for i in 5..lim {
        if sieve.access(i) {
            for j in (i*i..n+1).step_by(i*i) {
                sieve.set(j, false);
            }
        }
    }
    sieve.set(2, true);
    sieve.set(3, true);
    sieve.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_brute(n: usize) -> Vec<u64> {
        let mut primes = Vec::new();
        for i in 2..n+1 {
            if (2..i).all(|j| i % j > 0) {
                primes.push(i as u64);
            }
        }
        primes
    }

    #[test]
    fn test_atkin() {
        for i in (1..1000).step_by(100) {
            assert_eq!(atkin_sieve(i), prime_brute(i));
        }
    }
}
