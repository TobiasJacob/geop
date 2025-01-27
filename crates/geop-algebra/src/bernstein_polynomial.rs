use std::fmt::Display;

use crate::{
    bernstein_basis::BernsteinBasis, efloat::EFloat64, monomial_polynom::MonomialPolynom, HasZero,
    MultiDimensionFunction, OneDimensionFunction, ToMonomialPolynom,
};

// Represents a polynomial in the form of a_{0} B_{0,n}
pub struct BernsteinPolynomial<T> {
    coefficients: Vec<T>,
}

impl BernsteinPolynomial<EFloat64> {
    pub fn new(coefficients: Vec<EFloat64>) -> Self {
        Self { coefficients }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn from_monomial_polynom(monomial_polynom: MonomialPolynom) -> Self {
        let n = monomial_polynom.degree(); // Degree of the polynomial
        let mut bernstein_coeffs = vec![EFloat64::zero(); n + 1];

        for i in 0..=n {
            for k in 0..=i {
                let factor = (EFloat64::from(binomial_coefficient(i, k) as f64)
                    / EFloat64::from(binomial_coefficient(n, k) as f64))
                .unwrap();

                print!("{}\t", factor);
                bernstein_coeffs[i] =
                    bernstein_coeffs[i] + factor * monomial_polynom.monomials[k].clone();
            }
            println!();
        }

        Self::new(bernstein_coeffs)
    }

    pub fn to_monomial_polynom(&self) -> MonomialPolynom {
        let n = self.degree(); // Degree of the polynomial
        let mut monomial_coeffs = vec![EFloat64::zero(); n + 1];

        for i in 0..=n {
            for k in 0..=i {
                let factor = binomial_coefficient(n, i) * binomial_coefficient(i, k);
                let sign = if (i - k) % 2 == 0 { 1 } else { -1 };
                let factor = EFloat64::from(sign as f64) * EFloat64::from(factor as f64);
                print!("{}\t", factor);

                monomial_coeffs[i] = monomial_coeffs[i] + factor * self.coefficients[k].clone();
            }
            println!();
        }

        MonomialPolynom::new(monomial_coeffs)
    }
}

impl<T> BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    pub fn to_monomial_polynom_slow(&self) -> MonomialPolynom {
        let mut result = MonomialPolynom::zero();
        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BernsteinBasis::new(i, self.coefficients.len() - 1).unwrap();
            let basis_monomial = basis.to_monomial_polynom();
            let coeff = coeff.to_monomial_polynom();
            let term = &coeff * &basis_monomial;
            result = &result + &term;
        }

        result
    }
}

impl<T> MultiDimensionFunction<T> for BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    fn eval(&self, t: EFloat64) -> T {
        let mut result = T::zero();

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BernsteinBasis::new(i, self.coefficients.len() - 1).unwrap();
            let basis_eval = basis.eval(t);
            result = result + coeff.clone() * basis_eval;
        }

        result
    }
}

// Utility function for binomial coefficients
fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        0
    } else {
        (1..=k).fold(1, |acc, i| acc * (n + 1 - i) / i)
    }
}

impl Display for BernsteinPolynomial<EFloat64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        let n = self.degree();
        for (i, coeff) in self.coefficients.iter().enumerate() {
            if *coeff != EFloat64::zero() {
                if !first {
                    write!(f, " + ")?;
                }
                write!(f, "{} B_{{{},{}}}(t)", coeff, i, n)?;
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bernstein_to_monomial_conversion() {
        let coeffs = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(1.0),
            EFloat64::from(5.0),
            EFloat64::from(3.0),
        ];
        let bernstein = BernsteinPolynomial::new(coeffs.clone());
        let monomial = bernstein.to_monomial_polynom();
        let monomial2 = bernstein.to_monomial_polynom_slow();
        assert_eq!(monomial.monomials, monomial2.monomials);
        // println!("Bernstein Polynomial: {}", &bernstein);
        println!("Monomial Polynomial: {}", &monomial);
        println!("Monomial Polynomial (slow): {}", &monomial2);

        let back_to_bernstein = BernsteinPolynomial::from_monomial_polynom(monomial.clone());

        println!("Bernstein Polynomial: {}", &bernstein);
        println!(
            "Bernstein Polynomial (from monomial): {}",
            &back_to_bernstein
        );

        assert_eq!(back_to_bernstein.coefficients, coeffs);
    }

    #[test]
    fn test_monomial_to_bernstein_conversion() {
        let monomial_coeffs = MonomialPolynom::new(vec![
            EFloat64::from(3.0),
            EFloat64::from(-2.0),
            EFloat64::from(1.0),
        ]);

        let bernstein = BernsteinPolynomial::from_monomial_polynom(monomial_coeffs.clone());
        let back_to_monomial = bernstein.to_monomial_polynom();
        let back_to_monomial_slow = bernstein.to_monomial_polynom_slow();

        assert_eq!(back_to_monomial.monomials, monomial_coeffs.monomials);
        assert_eq!(back_to_monomial_slow.monomials, monomial_coeffs.monomials);
        println!("Bernstein Polynomial: {}", &bernstein);
    }
}
