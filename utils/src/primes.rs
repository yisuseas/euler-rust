// Prime number related functions

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
