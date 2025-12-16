//! The arithmetic sequence, 1487, 4817, 8147,
//! in which each of the terms increases by 3330, is unusual in two ways:
//! 1. each of the three terms are prime, and,
//! 2. each of the 4-digit numbers are permutations of one another.
//! There are no arithmetic sequences made up of
//! three 1-, 2-, or 3-digit primes, exhibiting this property,
//! but there is one other 4-digit increasing sequence.
//! What 12-digit number do you form by concatenating
//! the three terms in this sequence?

use std::collections::{HashMap, HashSet};
use utils::primes;

fn digit_map<T: std::fmt::Display>(number: T) -> HashMap<char, u8> {
    let mut map = HashMap::new();
    number.to_string().chars().for_each(|ch| {
        let counter = map.entry(ch).or_insert(0);
        *counter += 1;
    });
    map
}

fn are_permutations(a: usize, b: usize) -> bool {
    digit_map(a) == digit_map(b)
}

fn answer() -> u64 {
    println!("The arithmetic sequence, 1487, 4817, 8147,");
    println!("in which each of the terms increases by 3330,");
    println!("is unusual in two ways:");
    println!("1. each of the three terms are prime, and,");
    println!("2. each of the 4-digit numbers are permutations of one another.");
    println!("There are no arithmetic sequences made up of");
    println!("three 1-, 2-, or 3-digit primes, exhibiting this property,");
    println!("but there is one other 4-digit increasing sequence.");
    println!("What 12-digit number do you form by concatenating");
    println!("the three terms in this sequence?\n");

    let prime_vec: Vec<usize> = primes::primes_under(10_000)
        .iter()
        .filter_map(|&p| if p >= 1_000 { Some(p) } else { None })
        .collect();
    let prime_set: HashSet<usize> =
        HashSet::from_iter(prime_vec.iter().map(|&p| p));

    for i in 0..prime_vec.len() - 1 {
        let prime_i = prime_vec[i];
        for j in i + 1..prime_vec.len() {
            let prime_j = prime_vec[j];
            if are_permutations(prime_i, prime_j) {
                let diff = prime_j - prime_i;
                let prime_k = prime_j + diff;
                if prime_k >= 10_000 {
                    break;
                }
                if are_permutations(prime_j, prime_k)
                    && prime_set.contains(&prime_k)
                {
                    let sequence = [prime_i, prime_j, prime_k];
                    println!("{:?} -> diff: {}", sequence, diff);
                    if sequence != [1487, 4817, 8147] {
                        return prime_k as u64
                            + prime_j as u64 * 1_0000
                            + prime_i as u64 * 1_0000_0000;
                    }
                }
            }
        }
    }

    0
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e049_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 2969_6299_9629;
        assert_eq!(expected, answer());
    }
}
