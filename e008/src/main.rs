// The four adjacent digits in the 1000-digit number that have
// the greatest product are 9 × 9 × 8 × 9 = 5832.
// Find the thirteen adjacent digits in the 1000-digit number that have
// the greatest product. What is the value of this product?

fn answer() -> u64 {
  let adj_digits: usize = 13;

  let thousand_digit_number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
  let mut digit_array = [0u8; 1_000];
  {
    // Make the digit array
    let mut idx = 0;
    for digit in thousand_digit_number.chars() {
      let value = digit as u8 - 48;
      digit_array[idx] = value;
      idx += 1;
    }
  }

  let mut greatest_factors = &digit_array[0..adj_digits];
  let mut greatest_product = 0;
  for start in 0..(1_000 - adj_digits) {
    let factors = &digit_array[start..start + adj_digits];
    let mut product: u64 = 1;
    for factor in factors {
      product *= *factor as u64;
    }
    if product > greatest_product {
      greatest_product = product;
      greatest_factors = factors;
    }
  }

  println!(
    "The {} adjacent digits in the 1000-digit number",
    adj_digits
  );
  println!("that have the greatest product are: {:?}", greatest_factors);
  println!("What is the value of this product?");

  greatest_product
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e008_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 23514624000;
    assert_eq!(expected, answer());
  }
}
