//! In the United Kingdom the currency is made up of pound (£) and pence (p).
//! There are eight coins in general circulation:
//!     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
//! It is possible to make £2 in the following way:
//!     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//! How many different ways can £2 be made using any number of coins?

#[derive(Debug, Clone, Copy, PartialEq)]
enum Coin {
    Pence1 = 1,
    Pence2 = 2,
    Pence5 = 3,
    Pence10 = 10,
    Pence20 = 20,
    Pence50 = 50,
    Pound1 = 100,
    Pound2 = 200,
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Self::Pence1 => 1,
            Self::Pence2 => 2,
            Self::Pence5 => 5,
            Self::Pence10 => 10,
            Self::Pence20 => 20,
            Self::Pence50 => 50,
            Self::Pound1 => 100,
            Self::Pound2 => 200,
        }
    }

    fn options() -> [Coin; 8] {
        [
            Coin::Pence1,
            Coin::Pence2,
            Coin::Pence5,
            Coin::Pence10,
            Coin::Pence20,
            Coin::Pence50,
            Coin::Pound1,
            Coin::Pound2,
        ]
    }
}

fn ways_to_make(quant: u8, coins: &[Coin]) -> usize {
    let mut ways = 0;
    if quant == 0 || coins.len() == 1 {
        ways = 1;
    } else {
        let mut coins: Vec<Coin> = coins.iter().map(|&c| c).collect();
        let last = coins.pop().unwrap();

        for ammount in 0..=(quant / last.value()) {
            let remainder = quant - ammount * last.value();
            ways += ways_to_make(remainder, &coins);
        }
    }
    // println!("{} ways to make {} with {:?}", ways, quant, coins);
    ways
}

fn answer() -> usize {
    let coins = Coin::options();

    println!(
    "How many different ways can 2 pounds be made using any number of coins?"
  );

    ways_to_make(200, &coins)
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e031_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 73_682;
        assert_eq!(expected, answer());
    }
}
