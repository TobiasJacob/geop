use std::fmt::Display;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
    HasZero, MultiDimensionFunction, ToMonomialPolynom,
};

#[derive(Debug, Clone)]
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
    T: Display,
{
    /// Inserts the knot value `t` once into the B‑spline curve using the standard knot insertion algorithm.
    /// Returns a new BSplineCurve with an updated control net and knot vector.
    fn insert_knot(&self, t: EFloat64) -> AlgebraResult<BSplineCurve<T>> {
        // Find the span index k such that knot_vector[k] <= t < knot_vector[k+1]
        let k = match self.find_span(t.clone()) {
            Some(k) => k,
            None => {
                return Err(AlgebraError::new(
                    "Parameter t is out of the valid domain for knot insertion".to_string(),
                ))
            }
        };

        let p = self.degree;
        let n = self.coefficients.len() - 1;
        let mut new_coeffs = Vec::with_capacity(self.coefficients.len() + 1);
        let mut new_knots = Vec::with_capacity(self.knot_vector.len() + 1);

        // Build new knot vector: copy knots up to k (inclusive), insert t, then copy the remaining knots.
        for i in 0..=k {
            new_knots.push(self.knot_vector[i].clone());
        }
        new_knots.push(t.clone());
        for i in (k + 1)..self.knot_vector.len() {
            new_knots.push(self.knot_vector[i].clone());
        }

        // The new control points:
        // 1. Copy control points unaffected by the insertion.
        for i in 0..(k - p + 1) {
            new_coeffs.push(self.coefficients[i].clone());
        }
        // 2. Recompute the control points affected by the insertion.
        for i in (k - p + 1)..=k {
            // Compute alpha = (t - knot_vector[i]) / (knot_vector[i+p] - knot_vector[i])
            let alpha = ((t.clone() - self.knot_vector[i].clone())
                / (self.knot_vector[i + p].clone() - self.knot_vector[i].clone()))
            .unwrap_or(EFloat64::zero());

            // New control point: (1 - alpha)*P[i-1] + alpha*P[i]
            let new_point = self.coefficients[i - 1].clone() * (EFloat64::one() - alpha.clone())
                + self.coefficients[i].clone() * alpha;
            new_coeffs.push(new_point);
        }
        // 3. Copy the remaining control points.
        for i in k..=n {
            new_coeffs.push(self.coefficients[i].clone());
        }

        BSplineCurve::try_new(new_coeffs, new_knots, p)
    }

    /// Subdivides the B‑spline curve at parameter `t` into two new BSplineCurve segments.
    ///
    /// The method first inserts `t` repeatedly until its multiplicity equals degree+1 (i.e. a break point).
    /// Then it splits the control net and knot vector into a left segment (defined over [a, t])
    /// and a right segment (defined over [t, b]).
    pub fn subdivide(&self, t: EFloat64) -> AlgebraResult<(BSplineCurve<T>, BSplineCurve<T>)> {
        let p = self.degree;

        // Ensure t lies within the valid parameter domain.
        if t < self.knot_vector[p] || t > self.knot_vector[self.knot_vector.len() - p - 1] {
            return Err(AlgebraError::new(
                "Parameter t is out of the valid domain for subdivision".to_string(),
            ));
        }

        // Determine the current multiplicity of t in the knot vector.
        let current_multiplicity = self.knot_vector.iter().filter(|&knot| *knot == t).count();
        // To split the curve, t must appear with multiplicity p+1.
        let r = p - current_multiplicity;
        let mut curve: BSplineCurve<T> = self.clone();
        for _ in 0..r {
            curve = curve.insert_knot(t.clone())?;
        }

        // We need an index i such that knots[i - p] == t and knots[i] == t.
        let t_index = curve.find_span(t.clone());
        let t_index = match t_index {
            Some(idx) => idx,
            None => {
                return Err(AlgebraError::new(
                    "Failed to locate knot with full multiplicity after insertion".to_string(),
                ))
            }
        };

        // The left segment uses control points from 0 to (t_index - p) and knot vector from 0 to t_index.
        let left_ctrl_pts = curve.coefficients[..=(t_index - p)].to_vec();
        let left_knots = curve.knot_vector[..=t_index + 1].to_vec();
        let left_curve = BSplineCurve::try_new(left_ctrl_pts, left_knots, p)?;

        // The right segment uses control points from (t_index - p) to end and knot vector from t_index to end.
        let right_ctrl_pts = curve.coefficients[(t_index - p)..].to_vec();
        let right_knots = curve.knot_vector[t_index - p..].to_vec();
        let right_curve = BSplineCurve::try_new(right_ctrl_pts, right_knots, p)?;

        Ok((left_curve, right_curve))
    }

    pub fn eval_slow(&self, t: EFloat64) -> T {
        let mut result = T::zero();

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BSplineCurve::<EFloat64>::try_new_from_basis(
                i,
                self.degree,
                self.knot_vector.clone(),
            )
            .unwrap();
            result = result + coeff.clone() * basis.eval(t);
        }

        result
    }
}

impl<T> Display for BSplineCurve<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BSplineCurve(")?;
        for coeff in self.coefficients.iter() {
            write!(f, "{}, ", coeff)?;
        }
        write!(f, "degree: {}, ", self.degree)?;
        write!(f, "knots: [")?;
        for knot in &self.knot_vector {
            write!(f, "{}, ", knot)?;
        }
        write!(f, "])")
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
        let k = match self.find_span(t) {
            Some(k) => k,
            None => return T::zero(),
        };
        let p = self.degree;

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

                        // In case we divide by zero, the alpha value does not matter, so we choose 0.
                        ((t - left_knot) / (right_knot - left_knot)).unwrap_or(EFloat64::zero())
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

    #[test]
    fn test_bspline_knot_insertion() -> AlgebraResult<()> {
        // Create a B-spline curve with 2D points as coefficients
        let coefficients = vec![
            EFloat64::from(5.0),
            EFloat64::from(1.0),
            EFloat64::from(3.0),
            EFloat64::from(6.0),
            EFloat64::from(32.0),
            EFloat64::from(25.0),
            EFloat64::from(4.0),
            EFloat64::from(19.0),
        ];

        // Strictly increasing knot vector
        let knot_vector = to_efloat_vec(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0,
        ]);

        let bspline = BSplineCurve::try_new(coefficients, knot_vector, 3).unwrap();
        println!("bspline: {}", bspline);

        // Choose a subdivision parameter in the valid domain.
        let t = EFloat64::from(2.5);
        // Subdivide the B‑spline at t.
        let bspline2 = bspline.insert_knot(t.clone())?;

        // print
        println!("bspline2: {}", bspline2);

        for i in 0..=100 {
            let t = i as f64 / 100.0 * 7.0;
            let t = EFloat64::from(t);
            let result_eval = bspline.eval(t.clone());
            let result2_eval = bspline2.eval(t.clone());
            assert_eq!(result_eval, result2_eval);
        }

        Ok(())
    }

    #[test]
    fn test_bspline_subdivide1() -> AlgebraResult<()> {
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
        println!("bspline: {}", bspline);

        // Choose a subdivision parameter in the valid domain.
        let t = EFloat64::from(3.5);
        // Subdivide the B‑spline at t.
        let (left, right) = bspline.subdivide(t.clone())?;

        // print
        println!("left: {}", left);
        println!("right: {}", right);

        for i in 0..100 {
            let t = i as f64 / 100.0 * 3.5;
            let t = EFloat64::from(t);
            let result_eval = bspline.eval(t.clone());
            let result2_eval = left.eval(t.clone());
            assert_eq!(
                result_eval, result2_eval,
                "Left segment evaluation at t={} does not match the original curve",
                t
            );
        }

        for i in 0..100 {
            let t = i as f64 / 100.0 * 3.5 + 3.5;
            let t = EFloat64::from(t);
            let result_eval = bspline.eval(t.clone());
            let result2_eval = right.eval(t.clone());
            assert_eq!(
                result_eval, result2_eval,
                "Right segment evaluation at t={} does not match the original curve",
                t
            );
        }

        Ok(())
    }

    #[test]
    fn test_bspline_subdivide2() -> AlgebraResult<()> {
        // Create a cubic (degree 3) clamped B‑spline curve.
        // For a clamped B‑spline of degree 3 with 4 control points,
        // a common knot vector is [0, 0, 0, 0, 1, 1, 1, 1].
        let coefficients = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(4.0),
        ];
        let knot_vector = vec![
            EFloat64::from(0.0),
            EFloat64::from(0.0),
            EFloat64::from(0.0),
            EFloat64::from(0.5),
            EFloat64::from(1.0),
            EFloat64::from(1.0),
            EFloat64::from(1.0),
        ];
        let bspline = BSplineCurve::try_new(coefficients, knot_vector, 3).unwrap();

        // Choose a subdivision parameter in the valid domain.
        let t = EFloat64::from(0.5);
        // Subdivide the B‑spline at t.
        let (left, right) = bspline.subdivide(t.clone()).unwrap();

        // Evaluate the original curve at t.
        let orig_val = bspline.eval(t.clone());
        // Evaluate the left and right segments at t.
        let left_val = left.eval(t.clone());
        let right_val = right.eval(t.clone());

        // Check that the joining point matches.
        assert_eq!(
            orig_val, left_val,
            "Left segment evaluation at t={} does not match the original curve",
            t
        );
        assert_eq!(
            orig_val, right_val,
            "Right segment evaluation at t={} does not match the original curve",
            t
        );

        Ok(())
    }
}
