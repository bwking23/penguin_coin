use mod_exp::mod_exp;
use std::ops;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FiniteSetError {
    #[error("Number, {0}, is invalid as it is larger then the prime, {1}.")]
    NumberTooLarge(i64, i64),
    #[error("Number, {0}, is less then zero, which is invalid")]
    NumberLessThanZero(i64),
    #[error("The prime value must be equal to add. Provide primes {0} and {1}.")]
    MisMatchedPrimes(i64, i64),
}

type Result<T> = std::result::Result<T, FiniteSetError>;

#[derive(Debug, Eq, PartialEq)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Result<Self> {
        if num >= prime {
            return Err(FiniteSetError::NumberTooLarge(num, prime));
        }
        if num < 0 {
            return Err(FiniteSetError::NumberLessThanZero(num));
        }
        Ok(FieldElement { num, prime })
    }

    pub fn pow(self, exp: i64) -> Self {
        let n = exp % (self.prime - 1);
        let n = if n < 0 { n + (self.prime - 1) } else { n };
        let num = mod_exp(self.num, n, self.prime);
        FieldElement {
            num: if num < 0 { num + self.prime } else { num },
            prime: self.prime,
        }
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Result<Self> {
        if self.prime != other.prime {
            return Err(FiniteSetError::MisMatchedPrimes(self.prime, other.prime));
        }
        Ok(FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        })
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = Result<Self>;

    fn sub(self, other: Self) -> Result<Self> {
        if self.prime != other.prime {
            return Err(FiniteSetError::MisMatchedPrimes(self.prime, other.prime));
        }

        Ok(FieldElement {
            num: if self.num < other.num {
                (self.num - other.num) % self.prime + self.prime
            } else {
                (self.num - other.num) % self.prime
            },
            prime: self.prime,
        })
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = Result<Self>;

    fn mul(self, other: Self) -> Result<Self> {
        if self.prime != other.prime {
            return Err(FiniteSetError::MisMatchedPrimes(self.prime, other.prime));
        }
        let prod = self.num * other.num;
        Ok(FieldElement {
            num: if prod < 0 {
                prod % self.prime + self.prime
            } else {
                prod % self.prime
            },
            prime: self.prime,
        })
    }
}

impl ops::Div<FieldElement> for FieldElement {
    type Output = Result<Self>;

    fn div(self, other: Self) -> Result<Self> {
        if self.prime != other.prime {
            return Err(FiniteSetError::MisMatchedPrimes(self.prime, other.prime));
        }
        Ok(FieldElement {
            num: self.num * other.pow(self.prime - 2).num % self.prime,
            prime: self.prime,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_new() {
        assert_eq!(
            FieldElement::new(1, 5).unwrap(),
            FieldElement { num: 1, prime: 5 }
        );
        let too_large_error = FieldElement::new(7, 5).unwrap_err();
        assert_eq!(too_large_error, FiniteSetError::NumberTooLarge(7, 5));
        let less_than_zero_error = FieldElement::new(-1, 5).unwrap_err();
        assert_eq!(less_than_zero_error, FiniteSetError::NumberLessThanZero(-1));
    }

    #[test]
    fn test_field_element_equality() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(2, 31).unwrap();
        let c = FieldElement::new(15, 31).unwrap();
        assert!(a == b);
        assert!(a != c);
        assert!(!(a != b));
    }

    #[test]
    fn test_field_element_add() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(15, 31).unwrap();
        assert_eq!((a + b).unwrap(), FieldElement { num: 17, prime: 31 });
        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 31).unwrap();
        assert_eq!((a + b).unwrap(), FieldElement { num: 7, prime: 31 });
        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 37).unwrap();
        let mis_match_primes_error = (a + b).unwrap_err();
        assert_eq!(
            mis_match_primes_error,
            FiniteSetError::MisMatchedPrimes(31, 37)
        );
    }

    #[test]
    fn test_field_element_sub() {
        let a = FieldElement::new(29, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();
        assert_eq!((a - b).unwrap(), FieldElement { num: 25, prime: 31 });
        let a = FieldElement::new(15, 31).unwrap();
        let b = FieldElement::new(30, 31).unwrap();
        assert_eq!((a - b).unwrap(), FieldElement { num: 16, prime: 31 });
        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 37).unwrap();
        let mis_match_primes_error = (a - b).unwrap_err();
        assert_eq!(
            mis_match_primes_error,
            FiniteSetError::MisMatchedPrimes(31, 37)
        );
    }

    #[test]
    fn test_field_element_mul() {
        let a = FieldElement::new(24, 31).unwrap();
        let b = FieldElement::new(19, 31).unwrap();
        assert_eq!((a * b).unwrap(), FieldElement { num: 22, prime: 31 });
    }

    #[test]
    fn test_field_element_pow() {
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(3), FieldElement { num: 15, prime: 31 });
        let a = FieldElement::new(5, 31).unwrap();
        let b = FieldElement::new(18, 31).unwrap();
        assert_eq!((a.pow(5) * b).unwrap(), FieldElement { num: 16, prime: 31 });
    }

    #[test]
    fn test_field_element_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();
        assert_eq!((a / b).unwrap(), FieldElement { num: 4, prime: 31 });
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(-3), FieldElement { num: 29, prime: 31 });
        let a = FieldElement::new(4, 31).unwrap();
        let b = FieldElement::new(11, 31).unwrap();
        assert_eq!(
            (a.pow(-4) * b).unwrap(),
            FieldElement { num: 13, prime: 31 }
        );
    }
}
