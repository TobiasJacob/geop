use std::vec;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
};

pub struct BSplineBasis {
    index: usize,  // i in book
    degree: usize, // k in book
    knot_vector: Vec<EFloat64>,
}

impl BSplineBasis {
    pub fn new(
        index: usize,
        degree: usize,
        knot_vector: Vec<EFloat64>,
    ) -> AlgebraResult<BSplineBasis> {
        if index > degree {
            return Err(AlgebraError::new(format!(
                "BSplineBasis invalid input: index {} is greater than degree {}",
                index, degree
            )));
        }

        // Check knot vector is valid
        if knot_vector.len() != degree + 2 {
            return Err(AlgebraError::new(format!(
                "BSplineBasis invalid input: knot vector length {} is not equal to degree + 2 ({})",
                knot_vector.len(),
                degree + 2
            )));
        }

        // Non-decreasing knot vector
        for i in 1..knot_vector.len() {
            if !(knot_vector[i - 1] < knot_vector[i]) {
                return Err(AlgebraError::new(format!(
                    "BSplineBasis invalid input: knot vector is not decreasing at index {}",
                    i
                )));
            }
        }

        Ok(Self {
            index,
            degree,
            knot_vector,
        })
    }

    pub fn eval(&self, t: EFloat64) -> EFloat64 {
        // Use the de Casteljau algorithm to evaluate the Bernstein basis
        let mut b = vec![vec![EFloat64::zero(); self.degree + 1]; self.degree + 1];
        b[0][0] = EFloat64::one();
        for k in 1..=self.degree {
            if k == 1 {
                for i in 0..=self.degree {
                    if self.knot_vector[i] <= t && self.knot_vector[i + 1] > t {
                        b[i][1] = EFloat64::one();
                    } else {
                        b[i][1] = EFloat64::zero();
                    }
                }
                continue;
            }

            for i in 0..=k {
                if i == 0 {
                    b[i][k] = (EFloat64::one() - t) * b[i][k - 1];
                } else if i == k {
                    b[i][k] = t * b[i - 1][k - 1];
                } else {
                    let fac1 = (t - self.knot_vector[i])
                        / (self.knot_vector[i + self.degree - 1] - self.knot_vector[i]); // What to do if denominator is zero?
                    let fac2 = (self.knot_vector[i + self.degree] - t)
                        / (self.knot_vector[i + self.degree] - self.knot_vector[i + 1]);
                    b[i][k] = fac1.unwrap() * b[i][k - 1] + fac2.unwrap() * b[i + 1][k - 1];
                }
            }
        }
        b[self.index][self.degree]
    }

    // pub fn to_monomial_polynom(&self) -> MonomialPolynom {
    //     let fact1 = MonomialPolynom::new(vec![EFloat64::one(), -EFloat64::one()]);
    //     let fact2 = MonomialPolynom::new(vec![EFloat64::zero(), EFloat64::one()]);
    //     let coeff = binomial_coefficient(self.degree, self.index);
    //     let coeff = EFloat64::from(coeff as f64);
    //     let result = &(&fact1.pow(self.degree - self.index) * coeff) * &fact2.pow(self.index);
    //     result
    // }
}

impl PartialEq for BSplineBasis {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.degree == other.degree
            && self.knot_vector == other.knot_vector
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bernstein_basis() {
//         let b = BSplineBasis::new(3, 5);
//         let as_mon = b.to_monomial_polynom();
//         println!("{}", &as_mon);
//         for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
//             let t = EFloat64::from(t);
//             assert_eq!(b.eval(t), as_mon.eval(t));
//         }
//     }
//     #[test]
//     fn test_bernstein_basis2() {
//         let b = BSplineBasis::new(1, 5);
//         let as_mon = b.to_monomial_polynom();
//         println!("{}", &as_mon);
//         for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
//             let t = EFloat64::from(t);
//             assert_eq!(b.eval(t), as_mon.eval(t));
//         }
//     }
// }
