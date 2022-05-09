// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... 10^2 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the 
// first ten natural numbers and the square of the sum is
// 3025 - 385 = 2640
// Find the difference between the sum of the squares of the
// first one hundred natural numbers and the square of the sum.

fn main() {
    let n: u64 = 100;
    let sum_of_squares: u64 = (n * (n + 1) * (2 * n + 1)) / 6;
    let only_sum: u64 = (n * (n + 1)) / 2;
    let square_of_sum: u64 = only_sum * only_sum;
    let difference = square_of_sum - sum_of_squares;

    println!("The difference between the sum of the squares of");
    println!("the first {} natural numbers and the square of the sum is", n);
    println!("{} - {} = {}", square_of_sum, sum_of_squares, difference);
}
