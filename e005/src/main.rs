// 2520 is the smallest number that can be divided by each of the
// numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

use utils::primes;

fn main() {
    let mut v: Vec<u64> = Vec::new();
    for i in 2..=20 {
        v.push(i);
    }

    println!(
        "Given the numbers:\n{:?}\nThe lcm is:\n{}",
        &v,
        primes::least_common_multiple(&v)
    );
}
