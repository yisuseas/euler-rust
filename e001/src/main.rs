// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

fn sum_divisible_by(divisor: u64, target: u64) -> u64 {
    let p: u64 = (target - 1) / divisor;

    divisor * (p * (p + 1) / 2)
}

fn main() {
    // 1,000 fits in an u32, setting functions for u64 
    // in case a bigger number is needed
    let target: u64 = 1_000;
    let sum: u64 = sum_divisible_by(3, target)
                 + sum_divisible_by(5, target)
                 - sum_divisible_by(15, target);

    println!("The sum of all multiples of 3 or 5 bellow {} is \n{}", target, sum);
}
