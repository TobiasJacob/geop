use std::vec;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
    OneDimensionFunction,
};

pub struct BSplineBasis {
    index: usize,  // i in book
    degree: usize, // k in book
    knot_vector: Vec<EFloat64>,
}

// With 2 control points, we have B[0][0] as a step function
// With 3 control points, we have B[0][0] as a step function, B[1][0] as a step function, and B[1][1] as a linear function
// With 4 control points, we have B[0][0], B[1][0], B[2][0] as step functions, B[1][1], B[2][1] as linear functions, and B[2][2] as a quadratic function

// So for a valid Bespline, we have the following properties:
// - The knot vector is non-decreasing
// - Degree <= knot_vector.len() - 2
// - Index <= knot_vector.len() - degree - 2

impl BSplineBasis {
    pub fn new(
        index: usize,
        degree: usize,
        knot_vector: Vec<EFloat64>,
    ) -> AlgebraResult<BSplineBasis> {
        if degree > knot_vector.len() - 2 {
            return Err(AlgebraError::new(format!(
                "BSplineBasis invalid input: degree {} is greater than knot_vector.len() - 2 (len is {})",
                degree,
                knot_vector.len()
            )));
        }

        if index > knot_vector.len() - degree - 2 {
            return Err(AlgebraError::new(format!(
                "BSplineBasis invalid input: index {} is greater than knot_vector.len() ({}) - degree ({}) - (2)",
                index, knot_vector.len(), degree
            )));
        }

        // Non-decreasing knot vector
        for i in 1..knot_vector.len() {
            if !(knot_vector[i - 1] <= knot_vector[i]) {
                return Err(AlgebraError::new(format!(
                    "BSplineBasis invalid input: knot vector is not equal or increasing at index {}",
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
}

impl OneDimensionFunction for BSplineBasis {
    fn eval(&self, t: EFloat64) -> EFloat64 {
        // Use the de Casteljau algorithm to evaluate the Bernstein basis (this is slow)
        // What happens if we have two equal knots?
        // Lets say knot_vector[i1] to knot_vector[i2] are == t
        // Then the basis function will be zero for all t in [knot_vector[i1](t), knot_vector[i2 - 1]](t). It will be 1 at knot_vector[i2](t).
        // Hence we can assume that everything is 0 except for b[i2][0], b[i2 - 1][1], b[i2 - 2][2], ..., b[i1][degree]
        let mut b = vec![vec![EFloat64::zero(); self.degree + 1]; self.knot_vector.len() - 1];
        for i in 0..self.knot_vector.len() - 1 {
            if self.knot_vector[i] <= t && t < self.knot_vector[i + 1] {
                b[i][0] = EFloat64::one();
            } else {
                b[i][0] = EFloat64::zero();
            }
        }
        // Taken from https://en.wikipedia.org/wiki/B-spline#Definition with k=p
        for k in 1..=self.degree {
            for i in 0..self.knot_vector.len() - k - 1 {
                let fac1 =
                    (t - self.knot_vector[i]) / (self.knot_vector[i + k] - self.knot_vector[i]);
                // If the denominator is zero, it will be multiplied with 0, hence we can just set it to any arbitrary value. We choose zero.
                let fac1 = fac1.unwrap_or(EFloat64::zero());
                let fac2 = (self.knot_vector[i + k + 1] - t)
                    / (self.knot_vector[i + k + 1] - self.knot_vector[i + 1]);
                let fac2 = fac2.unwrap_or(EFloat64::zero());
                b[i][k] = fac1 * b[i][k - 1] + fac2 * b[i + 1][k - 1];
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

//     // #[test]
//     // fn test_bspline_basis() {
//     //     let b = BSplineBasis::new(0, 0, vec![EFloat64::from(0.0), EFloat64::from(1.0)]).unwrap();
//     //     let as_mon = b.to_monomial_polynom();
//     //     println!("{}", &as_mon);
//     //     for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
//     //         let t = EFloat64::from(t);
//     //         assert_eq!(b.eval(t), as_mon.eval(t));
//     //     }
//     // }
// }
