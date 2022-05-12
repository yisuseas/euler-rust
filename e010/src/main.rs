// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

use utils::primes;

fn main() {
    let target = 2_000_000;
    
    let prime_vec = primes::primes_under(target);
    let sum_of_primes = prime_vec
                            .iter()
                            .fold(0, |sum, &prime| sum + prime);

    println!(
        "The sum of primes below {} is:\n{}",
        target,
        sum_of_primes
    );
}
