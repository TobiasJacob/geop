use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    bspline_basis::BSplineBasis,
    efloat::EFloat64,
    HasZero, MultiDimensionFunction, OneDimensionFunction, ToMonomialPolynom,
};

pub struct BSplineCurve<T> {
    pub coefficients: Vec<T>,   // p_i in book, len(coefficients) = n
    knot_vector: Vec<EFloat64>, // t_k in book, len(knot_vector) = len(coefficients) + 1 + degree
    degree: usize,              // k in book
}

impl<T> BSplineCurve<T> {
    pub fn try_new(
        coefficients: Vec<T>,
        knot_vector: Vec<EFloat64>,
        degree: usize, // k in book
    ) -> AlgebraResult<Self> {
        if knot_vector.len() != coefficients.len() + 1 + degree {
            return Err(AlgebraError::new(format!(
                "BSplineCurve invalid input: knot_vector.len() ({}) != coefficients.len() ({}) + 1 + degree ({})",
                knot_vector.len(),
                coefficients.len(),
                degree
            )));
        }

        for i in 1..knot_vector.len() {
            if knot_vector[i - 1] > knot_vector[i] {
                return Err("Knot vector must be non-decreasing".into());
            }
        }

        Ok(Self {
            coefficients,
            knot_vector,
            degree,
        })
    }

    pub fn try_new_from_basis(
        index: usize,
        degree: usize,
        knot_vector: Vec<EFloat64>,
    ) -> AlgebraResult<BSplineCurve<EFloat64>> {
        if degree > knot_vector.len() - 2 {
            return Err(AlgebraError::new(format!(
                "BSplineCurve invalid input: degree {} is greater than knot_vector.len() - 2 (len is {})",
                degree,
                knot_vector.len()
            )));
        }

        if index > knot_vector.len() - degree - 2 {
            return Err(AlgebraError::new(format!(
                "BSplineCurve invalid input: index {} is greater than knot_vector.len() ({}) - degree ({}) - (2)",
                index, knot_vector.len(), degree
            )));
        }

        let mut coefficients = vec![EFloat64::zero(); knot_vector.len() - degree - 1];
        coefficients[index] = EFloat64::one();
        return BSplineCurve::<EFloat64>::try_new(coefficients, knot_vector, degree);
    }

    // k in book, needs to be >= 0
    pub fn degree(&self) -> usize {
        self.degree
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
        // Follows https://en.wikipedia.org/wiki/De_Boor%27s_algorithm
        let mut coeffs = self.coefficients.clone();
        let k = coeffs.len();
        let p = self.degree;
        for r in 1..=self.degree {
            for j in (self.degree..r - 1).rev() {
                let alpha = ((t - self.knot_vector[j + k - p].clone())
                    / (self.knot_vector[j + 1 + k - r].clone()
                        - self.knot_vector[j + k - p].clone()))
                .unwrap_or(EFloat64::zero()); // If denominator is zero, alpha value can be anything
                coeffs[j] =
                    coeffs[j - 1].clone() * (EFloat64::one() - alpha) + coeffs[j].clone() * alpha;
            }
        }
        return coeffs[p].clone();
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
            let basis = BSplineBasis::new(i, self.degree, self.knot_vector.clone()).unwrap();
            result = result + coeff.clone() * basis.eval(t);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efloat::EFloat64;

    // Helper function to convert Vec<f64> to Vec<EFloat64>
    fn to_efloat_vec(values: Vec<f64>) -> Vec<EFloat64> {
        values.into_iter().map(EFloat64::from).collect()
    }

    // Test for strictly increasing knot vector
    #[test]
    fn test_strictly_increasing_knot_vector() {
        // Create a B-spline curve with 2D points as coefficients
        let coefficients = vec![
            EFloat64::from(5.0),
            EFloat64::from(1.0),
            EFloat64::from(3.0),
            EFloat64::from(2.0),
        ];

        // Strictly increasing knot vector
        let knot_vector = to_efloat_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);

        let bspline = BSplineCurve::try_new(coefficients, knot_vector, 3).unwrap();

        // Test at various parameter values
        let test_params = to_efloat_vec(vec![1.5, 2.0, 2.5, 3.5, 4.5]);

        for t in test_params {
            let result_eval = bspline.eval(t);
            let result_fast = bspline.eval_fast(t);

            assert_eq!(result_eval, result_fast);
        }
    }
}
