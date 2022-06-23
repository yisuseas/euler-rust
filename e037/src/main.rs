//! The number 3797 has an interesting property. Being prime itself,
//! it is possible to continuously remove digits from left to right,
//! and remain prime at each stage: 3797, 797, 97, and 7.
//! Similarly we can work from right to left: 3797, 379, 37, and 3.
//! Find the sum of the only eleven primes that are both truncatable
//! from left to right and right to left.
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

use std::collections::HashSet;

fn cut_from_right(n: u64) -> Vec<u64> {
    let mut stages = Vec::new();
    let mut s = n.to_string();
    while s.len() > 1 {
        s.pop();
        stages.push(s.parse().unwrap());
    }
    stages
}

fn cut_from_left(n: u64) -> Vec<u64> {
    let mut stages = Vec::new();
    let n_str = &n.to_string()[..];
    for start in 1..n_str.len() {
        let s = &n_str[start..];
        stages.push(s.parse().unwrap());
    }
    stages
}

fn answer() -> u64 {
    println!("Find the sum of the only eleven primes that are both");
    println!("truncatable from left to right and right to left.\n");

    let mut prime_list = vec![2, 3];
    let mut prime_set = HashSet::from([2, 3]);
    let mut truncatable_list = Vec::new();

    let mut pos_prime = 5;
    while truncatable_list.len() < 11 {
        // Generate a prime
        let mut is_prime = true;
        for &prime in prime_list.iter() {
            if pos_prime % prime == 0 {
                is_prime = false;
                break;
            }
            if prime * prime > pos_prime {
                break;
            }
        }
        if is_prime {
            let prime = pos_prime;
            prime_list.push(prime);
            prime_set.insert(prime);
            // Check if truncatable
            if prime >= 10 {
                let mut truncatable = true;
                for n in cut_from_left(prime) {
                    if !prime_set.contains(&n) {
                        truncatable = false;
                        break;
                    }
                }
                if truncatable {
                    for n in cut_from_right(prime) {
                        if !prime_set.contains(&n) {
                            truncatable = false;
                            break;
                        }
                    }
                }
                if truncatable {
                    truncatable_list.push(prime);
                    println!("{prime}");
                }
            }
        }
        pos_prime += 2;
    }

    truncatable_list.iter().sum()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e037_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 748_317;
        assert_eq!(expected, answer());
    }
}
