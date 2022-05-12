// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let abc_sum: u32 = 1_000;
    
    let mut solved = false;

    for a in 1..((abc_sum - 3) / 3) {
        for b in (a + 1)..((abc_sum - 1 - a) / 2) {
            let c = abc_sum - a - b;
            if a*a + b*b == c*c {
                println!("abc_sum: {}", &abc_sum);
                println!("{} + {} + {} = {}", &a, &b, &c, a+b+c);
                println!("c^2 = {}", c*c);
                println!("a^2 + b^2 = {}", a*a + b*b);
                println!("a * b * c = {}", a*b*c);
                solved = true;
                break;
            }
        }
        if solved {
            break;
        }
    }
}
