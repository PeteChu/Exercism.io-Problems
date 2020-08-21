use std::iter::Iterator;

pub fn nth(n: u32) -> usize {
    const PRIME_SIZE: usize = 999999;
    let mut primes: [bool; PRIME_SIZE] = [true; PRIME_SIZE];

    for i in 2..primes.len() {
        if primes[i] {
            for j in (i..primes.len() - i).step_by(i) {
                primes[j+i] = false;
            }
        }
    }

    let mut c = 0;
    for v in 2..primes.len() {
        if primes[v] {
            if c == n {
                return v;
            }
            c += 1;
        }
    }

    return 0;
}