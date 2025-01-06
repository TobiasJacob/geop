use std::{fmt::Display, vec};

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
    factorial::binomial_coefficient,
    monomial_polynom::MonomialPolynom,
    OneDimensionFunction,
};

pub struct BernsteinBasis {
    index: usize,  // i in book
    degree: usize, // n in book
}

impl BernsteinBasis {
    pub fn new(index: usize, degree: usize) -> AlgebraResult<BernsteinBasis> {
        if index > degree {
            return Err(AlgebraError::new(format!(
                "BernsteinBasis invalid input: index {} is greater than degree {}",
                index, degree
            )));
        }
        Ok(Self { index, degree })
    }

    pub fn to_monomial_polynom(&self) -> MonomialPolynom {
        let fact1 = MonomialPolynom::new(vec![EFloat64::one(), -EFloat64::one()]);
        let fact2 = MonomialPolynom::new(vec![EFloat64::zero(), EFloat64::one()]);
        let coeff = binomial_coefficient(self.degree, self.index);
        let coeff = EFloat64::from(coeff as f64);
        let result = &(&fact1.pow(self.degree - self.index) * coeff) * &fact2.pow(self.index);
        result
    }
}

impl OneDimensionFunction for BernsteinBasis {
    fn eval(&self, t: EFloat64) -> EFloat64 {
        // Use the de Casteljau algorithm to evaluate the Bernstein basis
        let mut b = vec![vec![EFloat64::zero(); self.degree + 1]; self.degree + 1];
        b[0][0] = EFloat64::one();
        for n in 1..=self.degree {
            let max_i = n.min(self.degree + self.index - n);
            let min_i = 0;
            // TODO: The following line should work, but it seems to be wrong. Fix it.
            // let min_i = n.saturating_sub(self.index);
            for i in min_i..=max_i {
                if i == 0 {
                    b[i][n] = (EFloat64::one() - t) * b[i][n - 1];
                } else if i == n {
                    b[i][n] = t * b[i - 1][n - 1];
                } else {
                    b[i][n] = t * b[i - 1][n - 1] + (EFloat64::one() - t) * b[i][n - 1];
                }
            }
        }
        b[self.index][self.degree]
    }
}

impl PartialEq for BernsteinBasis {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.degree == other.degree
    }
}

impl Display for BernsteinBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B_{{{},{}}}(x)", self.index, self.degree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bernstein_basis() {
        let b = BernsteinBasis::new(3, 5).unwrap();
        let as_mon = b.to_monomial_polynom();
        println!("{}", &as_mon);
        for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
            let t = EFloat64::from(t);
            assert_eq!(b.eval(t), as_mon.eval(t));
        }
    }
    #[test]
    fn test_bernstein_basis2() {
        let b = BernsteinBasis::new(1, 5).unwrap();
        let as_mon = b.to_monomial_polynom();
        println!("{}", &as_mon);
        for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
            let t = EFloat64::from(t);
            assert_eq!(b.eval(t), as_mon.eval(t));
        }
    }
}
