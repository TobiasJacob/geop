use std::fmt::Display;

use crate::{efloat::EFloat64, monomial_polynom::MonomialPolynom, HasZero, MultiDimensionFunction};

// Represents a polynomial in the form of a_{0} B_{0,n}
pub struct BernsteinPolynomial<T> {
    pub coefficients: Vec<T>,
}

impl BernsteinPolynomial<EFloat64> {
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

    pub fn bernstein_basis(i: usize, n: usize) -> Self {
        let mut coefficients = vec![EFloat64::zero(); n + 1];
        coefficients[i] = EFloat64::one();
        Self::new(coefficients)
    }
}

impl<T> BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
{
    pub fn new(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    //$$ c_i^{n+r} = \sum_{j = max(0, i - r)}^{min(n, i)} \frac{\binom{r}{i - j} \binom{n}{j}}{\binom{n + r}{i}} c_i^n $$
    pub fn elevate_degree(&self, r: usize) -> Self {
        let n = self.degree();
        let mut new_coeffs = vec![T::zero(); n + r + 1];

        for i in 0..=n + r {
            for j in i.saturating_sub(r)..=n.min(i) {
                let factor = (EFloat64::from(
                    (binomial_coefficient(r, i - j) * binomial_coefficient(n, j)) as f64,
                ) / EFloat64::from(binomial_coefficient(n + r, i) as f64))
                .unwrap();
                new_coeffs[i] = new_coeffs[i].clone() + (self.coefficients[j].clone() * factor);
            }
        }

        Self::new(new_coeffs)
    }

    // Use de Casteljau's algorithm to subdivide the polynomial
    pub fn subdivide(&self, t: EFloat64) -> (BernsteinPolynomial<T>, BernsteinPolynomial<T>) {
        let mut beta = self.coefficients.clone();
        let n = beta.len();
        let mut left = vec![T::zero(); n];
        let mut right = vec![T::zero(); n];

        left[0] = beta[0].clone();
        right[n - 1] = beta[n - 1].clone();
        for j in 1..n {
            for k in 0..n - j {
                beta[k] = beta[k].clone() * (EFloat64::one() - t.clone())
                    + beta[k + 1].clone() * t.clone();
            }
            left[j] = beta[0].clone();
            right[n - j - 1] = beta[n - j - 1].clone();
        }

        (Self::new(left), Self::new(right))
    }
}

// From https://en.wikipedia.org/wiki/De_Casteljau%27s_algorithm
// def de_casteljau(t: float, coefs: list[float]) -> float:
//     """De Casteljau's algorithm."""
//     beta = coefs.copy()  # values in this list are overridden
//     n = len(beta)
//     for j in range(1, n):
//         for k in range(n - j):
//             beta[k] = beta[k] * (1 - t) + beta[k + 1] * t
//     return beta[0]
impl<T> MultiDimensionFunction<T> for BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
{
    fn eval(&self, t: EFloat64) -> T {
        let mut beta = self.coefficients.clone();
        let n = beta.len();
        for j in 1..n {
            for k in 0..n - j {
                beta[k] = beta[k].clone() * (EFloat64::one() - t.clone())
                    + beta[k + 1].clone() * t.clone();
            }
        }
        beta[0].clone()
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
    use crate::OneDimensionFunction;

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
        // println!("Bernstein Polynomial: {}", &bernstein);
        println!("Monomial Polynomial: {}", &monomial);

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

        assert_eq!(back_to_monomial.monomials, monomial_coeffs.monomials);
        println!("Bernstein Polynomial: {}", &bernstein);
    }

    #[test]
    fn test_bernstein_eval() {
        let coeffs = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(1.0),
            EFloat64::from(5.0),
            EFloat64::from(3.0),
        ];
        let bernstein = BernsteinPolynomial::new(coeffs.clone());
        let monomial = bernstein.to_monomial_polynom();

        let t = EFloat64::from(0.5);
        let eval = bernstein.eval(t);
        let eval_monomial = monomial.eval(t);
        assert_eq!(eval, eval_monomial);
        println!("Bernstein Polynomial: {}", &bernstein);
        println!("Bernstein Polynomial at {}: {}", t, eval);
    }

    #[test]
    fn test_bernstein_elevate_degree() {
        let coeffs = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(1.0),
            EFloat64::from(5.0),
            EFloat64::from(3.0),
        ];
        let bernstein = BernsteinPolynomial::new(coeffs.clone());

        let r = 2;
        let elevated_bernstein = bernstein.elevate_degree(r);

        println!("Bernstein Polynomial: {}", &bernstein);
        println!("Elevated Bernstein Polynomial: {}", &elevated_bernstein);

        for t in 0..=10 {
            let t = EFloat64::from(t as f64 / 10.0);
            assert_eq!(bernstein.eval(t), elevated_bernstein.eval(t), "t = {}", t);
        }
    }

    #[test]
    fn test_bernstein_elevate_degree2() {
        let coeffs = vec![EFloat64::from(1.0), EFloat64::from(2.0)];
        let bernstein = BernsteinPolynomial::new(coeffs.clone());

        println!("Bernstein Polynomial: {}", &bernstein);
        println!(
            "Elevated Bernstein Polynomial: {}",
            &bernstein.elevate_degree(1)
        );
        println!(
            "Elevated Bernstein Polynomial 2: {}",
            &bernstein.elevate_degree(2)
        );
    }

    #[test]
    fn test_bernstein_subdivide() {
        let coeffs = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(1.0),
            EFloat64::from(5.0),
            EFloat64::from(3.0),
        ];
        let bernstein = BernsteinPolynomial::new(coeffs.clone());

        let t = EFloat64::from(0.5);
        let (left, right) = bernstein.subdivide(t);

        println!("Bernstein Polynomial: {}", &bernstein);
        println!("Left Bernstein Polynomial: {}", &left);
        println!("Right Bernstein Polynomial: {}", &right);

        for t in 0..=10 {
            let t = EFloat64::from(t as f64 / 10.0);
            assert_eq!(
                bernstein.eval((t / EFloat64::two()).unwrap()),
                left.eval(t),
                "t = {}",
                t
            );
        }

        for t in 0..=10 {
            let t = EFloat64::from(t as f64 / 10.0);
            assert_eq!(
                bernstein.eval(((EFloat64::one() + t) / EFloat64::two()).unwrap()),
                right.eval(t),
                "t = {}",
                t
            );
        }
    }
}
