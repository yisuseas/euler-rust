// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
// What is the 10 001st prime number?

use utils::misc;

fn answer() -> u64 {
    let target: usize = 10_001;

    let mut prime_vec: Vec<u64> = vec![2, 3];
    
    // Find primes until we have enough
    while prime_vec.len() < target {
        let last_prime = prime_vec.last().unwrap();
        // Look through odd numbers until we find a prime
        let mut test_number = *last_prime + 2;
        loop {
            let mut is_prime = true;
            // Check if the test_number is divisible by any known prime
            for prime in &prime_vec {
                if *prime > (test_number as f64).sqrt() as u64 + 1 {
                    break;
                }
                if test_number % *prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                prime_vec.push(test_number);
                break;
            } else {
                test_number += 2;
            }
        }
    }

    println!(
        "What is the {} prime number?",
        misc::ordinal_number(target)
    );

    prime_vec.last().unwrap().clone()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e007_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 104743;
        assert_eq!(expected, answer());
    }
}
