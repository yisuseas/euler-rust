//! Pentagonal numbers are generated by the formula,
//!     Pn=n(3n−1)/2.
//! The first ten pentagonal numbers are:
//!     1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...
//! It can be seen that P4 + P7 = 22 + 70 = 92 = P8.
//! However, their difference, 70 − 22 = 48, is not pentagonal.
//! Find the pair of pentagonal numbers, Pj and Pk,
//! for which their sum and difference are pentagonal and
//! D = |Pk − Pj| is minimised; what is the value of D?

use std::collections::{HashSet};

/// The HashMap has the pentagonals as keys, and their indeces as values
struct PentaCache {
    numbers: Vec<u64>,
    number_set: HashSet<u64>,
}

impl PentaCache {
    fn new() -> PentaCache {
        PentaCache { numbers: Vec::new(), number_set: HashSet::new() }
    }

    fn find_next(&mut self) {
        let n = self.numbers.len() as u64 + 1;
        let penta = n * (3 * n - 1) / 2;
        self.numbers.push(penta);
        self.number_set.insert(penta);
    }

    fn last(&self) -> u64 {
        self.numbers.last().unwrap().clone()
    }

    fn is_pentagonal(&self, n: u64) -> bool {
        self.number_set.contains(&n)
    }
}

#[derive(Debug)]
struct PentagonalPair {
    pj: u64,
    pk: u64,
    dif: u64,
}

impl PentagonalPair {
    fn new() -> PentagonalPair {
        PentagonalPair { pj: 0, pk: 0, dif: u64::MAX }
    }

    fn set(&mut self, pj: u64, pk: u64, dif: u64) {
        self.pj = pj;
        self.pk = pk;
        self.dif = dif;
    }
}

fn answer() -> u64 {
    let mut penta_cache = PentaCache::new();
    let mut penta_pair = PentagonalPair::new();

    // Start with the first 2
    for _ in 0..2 {
        penta_cache.find_next();
    }

    let mut k = 1;
    loop {
        if k == penta_cache.numbers.len() {
            penta_cache.find_next();
        }
        let pk = penta_cache.numbers[k];
        // let next = penta_cache.numbers[k + 1];
        let n = k as u64 + 2;
        let next = n * (3 * n - 1) / 2;
        // println!("{pk} -> {next}");
        if next - pk > penta_pair.dif {
            break;
        }
        for j in (0..k).rev() {
            let pj = penta_cache.numbers[j];
            let dif = pk - pj;
            let sum = pj + pk;
            // if the diff won't be smaller than the one stored,
            // there's no need to keep looking
            if dif > penta_pair.dif {
                break;
            }
            if penta_cache.is_pentagonal(dif) {
                // Find necesary pentagonals for checking the sum
                while penta_cache.last() < sum {
                    penta_cache.find_next();
                }
                if penta_cache.is_pentagonal(sum) {
                    // We found one
                    penta_pair.set(pj, pk, dif);
                }
            }
        }
        k += 1;
    }

    println!("Find the pair of pentagonal numbers, Pj and Pk,");
    println!("for which their sum and difference are pentagonal and");
    println!("D = |Pk - Pj| is minimised; what is the value of D?");

    println!("\n{penta_pair:?}");

    penta_pair.dif
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e044_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 5482660;
        assert_eq!(expected, answer());
    }
}
