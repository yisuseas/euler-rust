use super::*;


use std::cmp::Ordering;

impl PartialOrd for Fraction {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.decimal().partial_cmp(&rhs.decimal())
    }
}


impl Fraction {
    fn plus(&self, rhs: &Fraction) -> Fraction {
        Fraction {
            num: self.num * rhs.den + rhs.num * self.den,
            den: self.den * rhs.den
        }
    }

    fn minus(&self, rhs: &Fraction) -> Fraction {
        Fraction {
            num: self.num * rhs.den - rhs.num * self.den,
            den: self.den * rhs.den
        }
    }

    fn times(&self, rhs: &Fraction) -> Fraction {
        Fraction {
            num: self.num * rhs.num,
            den: self.den * rhs.den
        }
    }

    fn over(&self, rhs: &Fraction) -> Fraction {
        Fraction {
            num: self.num * rhs.den,
            den: self.den * rhs.num
        }
    }
}


use std::ops::{
    Add, AddAssign,
    Mul, MulAssign,
    Sub, SubAssign,
    Div, DivAssign
};


impl Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}


impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}


impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}


impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.times(&rhs);
    }
}


impl Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}


impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(&rhs);
    }
}


impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.over(&rhs)
    }
}


impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.over(&rhs);
    }
}
