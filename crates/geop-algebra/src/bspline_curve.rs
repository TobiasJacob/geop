use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    bspline_basis::BSplineBasis,
    efloat::EFloat64,
    HasZero, MultiDimensionFunction, OneDimensionFunction, ToMonomialPolynom,
};

pub struct BSplineCurve<T> {
    pub coefficients: Vec<T>,   // p_i in book
    knot_vector: Vec<EFloat64>, // t_k in book
}

impl<T> BSplineCurve<T> {
    pub fn try_new(coefficients: Vec<T>, knot_vector: Vec<EFloat64>) -> AlgebraResult<Self> {
        // Check that the number of coefficients is at least 2
        if coefficients.len() != knot_vector.len() {
            return Err(AlgebraError::new(format!(
                "Number of coefficients ({}) must be one less than the number of knots ({})",
                coefficients.len(),
                knot_vector.len()
            )));
        }

        // Check that knot vector is non-decreasing
        for i in 1..knot_vector.len() {
            if knot_vector[i] > knot_vector[i - 1] {
                return Err("Knot vector must be non-decreasing".into());
            }
        }

        // Call BSplineBasis::new to create the basis - this will perform additional validations
        BSplineBasis::new(0, coefficients.len(), knot_vector.clone())?;

        Ok(Self {
            coefficients,
            knot_vector,
        })
    }
}

impl<T> BSplineCurve<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    pub fn eval_fast(&self, t: EFloat64) -> T {
        let coeffs = self.coefficients.clone();
        todo!()
    }
}

impl<T> MultiDimensionFunction<T> for BSplineCurve<T>
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
            let basis =
                BSplineBasis::new(i, self.coefficients.len(), self.knot_vector.clone()).unwrap();
            result = result + coeff.clone() * basis.eval(t);
        }

        result
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::efloat::EFloat64;

//     // Helper function to convert Vec<f64> to Vec<EFloat64>
//     fn to_efloat_vec(values: Vec<f64>) -> Vec<EFloat64> {
//         values.into_iter().map(EFloat64::from).collect()
//     }

//     // Test for strictly increasing knot vector
//     #[test]
//     fn test_strictly_increasing_knot_vector() {
//         // Create a B-spline curve with 2D points as coefficients
//         let coefficients = vec![
//             EFloat64::from(5.0),
//             EFloat64::from(1.0),
//             EFloat64::from(3.0),
//             EFloat64::from(4.0),
//         ];

//         // Strictly increasing knot vector
//         let knot_vector = to_efloat_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

//         let bspline = BSplineCurve::try_new(coefficients, knot_vector).unwrap();

//         // Test at various parameter values
//         let test_params = to_efloat_vec(vec![1.5, 2.0, 2.5, 3.5, 4.5]);

//         for t in test_params {
//             let result_eval = bspline.eval(t);
//             let result_fast = bspline.eval_fast(t);

//             assert_eq!(result_eval, result_fast);
//         }
//     }
// }
