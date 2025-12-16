//! The first two consecutive numbers to
//! have two distinct prime factors are:
//! 14 = 2 × 7
//! 15 = 3 × 5
//! The first three consecutive numbers to
//! have three distinct prime factors are:
//! 644 = 2^2 × 7 × 23
//! 645 = 3 × 5 × 43
//! 646 = 2 × 17 × 19.
//! Find the first four consecutive integers to
//! have four distinct prime factors each.
//! What is the first of these numbers?

use utils::primes;

fn distinct_pf(n: usize) -> usize {
    let pf_hm = primes::prime_factors_hm(n);
    pf_hm.keys().len()
}

fn answer() -> usize {
    println!("Find the first four consecutive integers to");
    println!("have four distinct prime factors each.");
    println!("What is the first of these numbers?");

    let req_quantity = 4;
    let req_consecutive = 4;

    let mut consecutive = 0;
    let mut n = 2;
    loop {
        if distinct_pf(n) == req_quantity {
            consecutive += 1;
        } else {
            consecutive = 0
        }
        if consecutive == req_consecutive {
            let answer = n - (req_consecutive - 1);
            println!("\n{} - {}", answer, n);
            return answer;
        }
        n += 1;
    }
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e047_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 134_043;
        assert_eq!(expected, answer());
    }
}
