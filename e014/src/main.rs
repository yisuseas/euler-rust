// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence
// (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem),
// it is thought that all starting numbers finish at 1.
// Which starting number, under one million, produces the longest chain?
// NOTE: Once the chain starts the terms are allowed to go above one million.

use std::collections::HashMap;

struct Collatz {
    known: HashMap<u64, u64>
}

impl Collatz {
    fn new() -> Collatz {
        Collatz {
            known: HashMap::from([(1, 1)])
        }
    }

    fn count_chain(&mut self, n: u64) -> u64 {
        if self.known.contains_key(&n) {
            return self.known[&n];
        }
    
        if n % 2 == 0 {
            let value = 1 + self.count_chain(n / 2);
            self.known.insert(n, value);
        } else {
            // 3n + 1 will be even, so we count both
            let value = 2 + self.count_chain((3 * n + 1) / 2);
            self.known.insert(n, value);
        }
    
        self.known[&n]
    }
}

fn answer() -> u64 {
    let target = 1_000_000;
    let mut longest_chain = 0;
    let mut answer = 0;

    let mut c = Collatz::new();

    // All the first half can be ignored
    // because we will calculate it in the even numbers of the second
    for starter in target/2..target {
        let this_chain = c.count_chain(starter);
        if this_chain > longest_chain {
            longest_chain = this_chain;
            answer = starter;
        }
    }

    println!("Find the greatest Collatz Sequence");
    println!("for starting numbers under {}.", target);
    println!("Which number produces it?");

    answer
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 837799;
        assert_eq!(expected, answer());
    }
}
