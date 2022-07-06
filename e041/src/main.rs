//! We shall say that an n-digit number is pandigital if it makes
//! use of all the digits 1 to n exactly once.
//! For example', '2143 is a 4-digit pandigital and is also prime.
//! What is the largest n-digit pandigital prime that exists?

use utils::{primes, misc};

struct PrimeCache {
    prime_list: Vec<usize>
}

impl PrimeCache {
    fn new() -> PrimeCache {
        let target = (1_000_000_000 as f64).sqrt() as usize;
        let prime_list = primes::primes_under(target);
        PrimeCache { prime_list }
    }

    fn is_prime(&self, number: usize) -> bool {
        for &prime in self.prime_list.iter() {
            if number % prime == 0 {
                return false;
            }
            if prime * prime > number {
                break;
            }
        }
        true
    }
}

fn answer() -> usize {   
    println!("What is the largest n-digit pandigital prime that exists?");
    
    let p_cache = PrimeCache::new();
    let ordered_digits = ['9', '8', '7', '6', '5', '4', '3', '2', '1'];

    for digit_count in (1..=9).rev() {
        // See if a prime can be made with these digits
        let digits_sum: usize = (1..=digit_count).sum();
        if digits_sum % 3 == 0 {
            continue;
        }
        // Try permutations of the digits from highest to lowest
        let digits = &ordered_digits[9 - digit_count..];
        for p in misc::permutations(digits) {
            let number: String = p.iter().collect();
            let number: usize = number.parse().unwrap();
            if p_cache.is_prime(number) {
                // Return the first pandigital prime found
                return number;
            }
        }
    }

    panic!("Couln't find a pandigital prime");
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e041_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 7652413;
        assert_eq!(expected, answer());
    }
}
