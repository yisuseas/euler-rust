// Prime number related functions

use std::collections::HashMap;

pub fn primes_under(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();

    if n > 2 {
        let mut all_numbers = vec![true; (n - 2) as usize];

        // println!("{} / {}", all_numbers.capacity(), isize::MAX);

        for number in 2..(n as f64).sqrt() as u64 + 1 {
            let number_idx = (number - 2) as usize;
            if all_numbers[number_idx] {
                let mut multiple = number * number;
                while multiple < n {
                    let multiple_idx = (multiple - 2) as usize;
                    all_numbers[multiple_idx] = false;
                    multiple += number;
                }
            }
        }

        for (idx, is_prime) in all_numbers.into_iter().enumerate() {
            if is_prime {
                let prime_number = (idx + 2) as u64;
                primes.push(prime_number);
            }
        }
    }

    primes
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let primes_under_n = primes_under((n as f64).sqrt() as u64 + 2);

    let mut quotient = n;
    for prime in &primes_under_n {
        loop {
            if quotient % prime == 0 {
                quotient = quotient / prime;
                prime_factors.push(*prime);
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

pub fn prime_factors_hm(n: u64) -> HashMap<u64, u16> {
    let mut pf: HashMap<u64, u16> = HashMap::new();
    
    let primes_under_n = primes_under(
        (n as f64).sqrt() as u64 + 2
    );

    let mut quotient = n;
    for prime in &primes_under_n {
        loop {
            if quotient % prime == 0 {
                let exp: &mut u16 = pf
                                   .entry(*prime)
                                   .or_insert(0u16);
                quotient = quotient / prime;
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

pub fn least_common_multiple(numbers: &Vec<u64>) -> u64 {
    // Find the biggest number
    let mut biggest: u64 = 0;
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
    let mut quotients = numbers.clone();
    let mut prime_factors: Vec<u64> = Vec::new();

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
    let mut lcm: u64 = 1;
    for pf in prime_factors {
        lcm *= pf;
    }

    lcm
}
