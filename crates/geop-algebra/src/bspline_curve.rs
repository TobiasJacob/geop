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

    fn find_span(&self, t: EFloat64) -> Option<usize> {
        // Handle edge cases
        if t < self.knot_vector[0] {
            return None;
        }
        if t >= self.knot_vector[self.knot_vector.len() - 1] {
            return None;
        }

        // Linear search to find the correct span
        let mut mid = 0;
        while !(self.knot_vector[mid] <= t && t < self.knot_vector[mid + 1]) {
            mid += 1;
        }

        // // Binary search to find the correct span
        // let mut low = 0;
        // let mut high = self.knot_vector.len() - 1;
        // let mut mid = (low + high) / 2;

        // while t < self.knot_vector[mid] || t >= self.knot_vector[mid + 1] {
        //     if t < self.knot_vector[mid] {
        //         high = mid;
        //     } else {
        //         low = mid;
        //     }
        //     mid = (low + high) / 2;
        // }

        Some(mid)
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
    pub fn eval_slow(&self, t: EFloat64) -> T {
        let mut result = T::zero();

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BSplineBasis::new(i, self.degree, self.knot_vector.clone()).unwrap();
            result = result + coeff.clone() * basis.eval(t);
        }

        result
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
        // Follows https://en.wikipedia.org/wiki/De_Boor%27s_algorithm

        // Find which knot span contains t
        let k = self.find_span(t);
        let p = self.degree;

        let k = match k {
            Some(k) => k,
            None => return T::zero(),
        };

        // Initialize coefficients for the de Boor algorithm
        let mut d = Vec::with_capacity(p + 1);

        // Ensure we're not accessing out of bounds indices
        for j in 0..=p {
            if k + j < p || k + j - p >= self.coefficients.len() {
                d.push(T::zero());
            } else {
                let idx = k + j - p;
                d.push(self.coefficients[idx].clone());
            }
        }

        // Apply de Boor's algorithm
        for r in 1..=p {
            for j in (r..=p).rev() {
                let alpha = match k + j < p || j + 1 + k - r >= self.knot_vector.len() {
                    true => EFloat64::zero(),
                    false => {
                        let left_knot = self.knot_vector[j + k - p].clone();
                        let right_knot = self.knot_vector[j + 1 + k - r].clone();

                        // Avoid division by zero
                        if left_knot == right_knot {
                            EFloat64::zero()
                        } else {
                            ((t - left_knot) / (right_knot - left_knot)).unwrap_or(EFloat64::zero())
                        }
                    }
                };
                d[j] = d[j - 1].clone() * (EFloat64::one() - alpha) + d[j].clone() * alpha;
            }
        }

        d[p].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efloat::EFloat64;

    fn to_efloat_vec(values: Vec<f64>) -> Vec<EFloat64> {
        values.into_iter().map(EFloat64::from).collect()
    }

    // Test for strictly increasing knot vector
    #[test]
    fn test_values_equal() {
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
            let result_2 = bspline.eval_slow(t);

            assert_eq!(result_eval, result_2);
        }
    }
}
