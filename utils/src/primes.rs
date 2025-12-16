//! Prime number related functions

use std::collections::HashMap;

/// Returns a vector of all primes under a given non-inclusive limit.
///
/// Uses Sieve of Eratostenes
pub fn primes_under(n: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    if n > 2 {
        let mut all_numbers = vec![true; n - 2];

        for number in 2..(n as f64).sqrt() as usize + 1 {
            let number_idx = number - 2;
            if all_numbers[number_idx] {
                let mut multiple = number * number;
                while multiple < n {
                    let multiple_idx = multiple - 2;
                    all_numbers[multiple_idx] = false;
                    multiple += number;
                }
            }
        }

        for (idx, is_prime) in all_numbers.into_iter().enumerate() {
            if is_prime {
                let prime_number = idx + 2;
                primes.push(prime_number);
            }
        }
    }

    primes
}

/// Returns a vector of the prime factors of a given integer,
///
/// Repeating factors, will appear as many times as necesary
pub fn prime_factors_vec(n: usize) -> Vec<usize> {
    let mut prime_factors: Vec<usize> = Vec::new();
    let primes_under_n = primes_under((n as f64).sqrt() as usize + 2);

    let mut quotient = n;
    for &prime in &primes_under_n {
        loop {
            if quotient % prime == 0 {
                quotient /= prime;
                prime_factors.push(prime);
            } else {
                break;
            }
        }
        if quotient == 1 {
            break;
        }
    }
    if quotient > 1 {
        prime_factors.push(quotient);
    }

    prime_factors
}

/// Returns a HashMap with the prime factors of a given number as keys,
/// and their exponent as values.
pub fn prime_factors_hm(n: usize) -> HashMap<usize, u32> {
    let mut pf: HashMap<usize, u32> = HashMap::new();

    let primes_under_n = primes_under((n as f64).sqrt() as usize + 2);

    let mut quotient = n;
    for &prime in &primes_under_n {
        loop {
            if quotient % prime == 0 {
                let exp: &mut u32 = pf.entry(prime).or_insert(0u32);
                quotient /= prime;
                *exp += 1;
            } else {
                break;
            }
        }
        if quotient == 1 {
            break;
        }
    }
    if quotient > 1 {
        pf.insert(quotient, 1);
    }

    pf
}

/// Returns the least common multiple of a Vector of integers
pub fn least_common_multiple(numbers: &[usize]) -> usize {
    // Find the biggest number
    let mut biggest: usize = 0;
    for n in numbers {
        if n > &biggest {
            biggest = *n;
        }
    }
    // Find all primes up to the biggest number
    let prime_vec = primes_under(biggest + 1);
    // Divide all numbers by each off the primes,
    // and storing the primes that divide at least one
    // number untill all of them are 1
    let mut quotients: Vec<usize> = numbers.iter().map(|&n| n).collect();
    let mut prime_factors: Vec<usize> = Vec::new();

    for prime in prime_vec {
        let mut all_are_ones = true;
        loop {
            let mut divides_smt = false;
            for q in &mut quotients {
                if *q % prime == 0 {
                    *q /= prime;
                    divides_smt = true;
                }
                if *q > 1 {
                    all_are_ones = false;
                }
            }
            if divides_smt {
                prime_factors.push(prime);
            } else {
                break;
            }
        }
        if all_are_ones {
            break;
        }
    }
    // Multiply all prime factors stored to get the lcm
    let mut lcm: usize = 1;
    for pf in prime_factors {
        lcm *= pf;
    }

    lcm
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::primes;
    use std::collections::HashMap;

    #[test]
    fn primes_under() {
        let empty_vec: Vec<usize> = Vec::new();
        assert_eq!(empty_vec, primes::primes_under(2));
        assert_eq!(vec![2, 3, 5, 7], primes::primes_under(10));
    }

    #[test]
    fn prime_factors_vec() {
        assert_eq!(vec![2, 2, 5], primes::prime_factors_vec(20));
        assert_eq!(vec![2, 2, 5, 23], primes::prime_factors_vec(460));
    }

    #[test]
    fn prime_factors_hm() {
        assert_eq!(
            HashMap::from([(2, 2), (5, 1)]),
            primes::prime_factors_hm(20)
        );
        assert_eq!(
            HashMap::from([(2, 2), (5, 1), (23, 1)]),
            primes::prime_factors_hm(460)
        );
    }

    #[test]
    fn least_common_multiple() {
        assert_eq!(60, primes::least_common_multiple(&vec![2, 5, 1, 3, 4]));
        assert_eq!(84, primes::least_common_multiple(&vec![4, 7, 12, 21, 42]));
    }
}
