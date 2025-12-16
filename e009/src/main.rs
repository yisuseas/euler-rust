// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn answer() -> u32 {
    let abc_sum: u32 = 1_000;

    let mut product = 0;

    for a in 1..((abc_sum - 3) / 3) {
        for b in (a + 1)..((abc_sum - 1 - a) / 2) {
            let c = abc_sum - a - b;
            if a * a + b * b == c * c {
                println!("a: {}\nb: {}\nc: {}", &a, &b, &c);
                product = a * b * c;
            }
        }
        if product > 0 {
            break;
        }
    }

    println!(
    "There exists exactly one Pythagorean triplet for which a + b + c = 1000."
  );
    println!("Find the product abc.");

    product
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e009_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 31875000;
        assert_eq!(expected, answer());
    }
}
