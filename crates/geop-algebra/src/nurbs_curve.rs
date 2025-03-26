use std::fmt::Display;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    bspline_curve::BSplineCurve,
    convex_hull::ConvexHull,
    efloat::EFloat64,
    point::Point,
    triangle::{quickhull, TriangleFace},
    HasZero, MultiDimensionFunction, ToMonomialPolynom,
};

/// A NURBS (Non-Uniform Rational B-Spline) curve.
///
/// * `coefficients` are the control points (Pᵢ).
/// * `weights` are the associated weights (wᵢ).
/// * `knot_vector` is the non-decreasing knot sequence.
/// * `degree` is the polynomial degree (p).
#[derive(Debug, Clone)]
pub struct NurbsCurve<T> {
    pub coefficients: Vec<T>,
    pub weights: Vec<EFloat64>,
    knot_vector: Vec<EFloat64>,
    degree: usize,
}

impl<T> NurbsCurve<T> {
    /// Create a new NURBS curve.
    ///
    /// Checks that:
    /// - The number of control points equals the number of weights.
    /// - The knot vector length equals `coefficients.len() + 1 + degree`.
    /// - The knot vector is non-decreasing.
    pub fn try_new(
        coefficients: Vec<T>,
        weights: Vec<EFloat64>,
        knot_vector: Vec<EFloat64>,
        degree: usize,
    ) -> AlgebraResult<Self> {
        if coefficients.len() != weights.len() {
            return Err(AlgebraError::new(format!(
                "Number of coefficients ({}) must equal number of weights ({})",
                coefficients.len(),
                weights.len()
            )));
        }
        if knot_vector.len() != coefficients.len() + 1 + degree {
            return Err(AlgebraError::new(format!(
                "NURBSCurve invalid input: knot_vector.len() ({}) != coefficients.len() ({}) + 1 + degree ({})",
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
            weights,
            knot_vector,
            degree,
        })
    }

    /// Create a NURBS basis function (with unit weights) for the given index.
    ///
    /// This sets the coefficient at the given index to one and all others to zero.
    pub fn try_new_from_basis(
        index: usize,
        degree: usize,
        knot_vector: Vec<EFloat64>,
    ) -> AlgebraResult<NurbsCurve<EFloat64>>
    where
        EFloat64: Clone + HasZero,
    {
        let n = knot_vector.len() - degree - 1;
        if index >= n {
            return Err(AlgebraError::new(format!(
                "NURBSCurve invalid input: index {} is out of range for knot_vector.len() {} and degree {}",
                index, knot_vector.len(), degree
            )));
        }
        let mut coefficients = vec![EFloat64::zero(); n];
        coefficients[index] = EFloat64::one();
        let weights = vec![EFloat64::one(); n];
        NurbsCurve::try_new(coefficients, weights, knot_vector, degree)
    }

    /// Returns the degree of the NURBS curve.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Finds the knot span index for a given parameter `t`.
    fn find_span(&self, t: EFloat64) -> Option<usize> {
        if t < self.knot_vector[0] {
            return None;
        }
        if t >= self.knot_vector[self.knot_vector.len() - 1] {
            return None;
        }
        let mut mid = 0;
        while !(self.knot_vector[mid] <= t && t < self.knot_vector[mid + 1]) {
            mid += 1;
        }
        Some(mid)
    }

    /// Subdivides the NURBS curve at parameter `t` into two new NurbsCurve segments.
    ///
    /// The method first inserts `t` repeatedly until its multiplicity equals degree+1 (i.e. a break point).
    /// Then it splits the control net and knot vector into a left segment (defined over [a, t])
    /// and a right segment (defined over [t, b]).
    pub fn subdivide(&self, t: EFloat64) -> AlgebraResult<(Self, Self)>
    where
        T: Clone,
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<EFloat64, Output = T>,
        T: std::ops::Div<EFloat64, Output = AlgebraResult<T>>,
        T: HasZero,
        T: ToMonomialPolynom,
    {
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
        let mut curve: NurbsCurve<T> = self.clone();
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

        // The left segment uses control points and weights from 0 to (t_index - p) and knot vector from 0 to t_index.
        let left_ctrl_pts = curve.coefficients[..=(t_index - p)].to_vec();
        let left_weights = curve.weights[..=(t_index - p)].to_vec();
        let left_knots = curve.knot_vector[..=t_index + 1].to_vec();
        let left_curve = NurbsCurve::try_new(left_ctrl_pts, left_weights, left_knots, p)?;

        // The right segment uses control points and weights from (t_index - p) to end and knot vector from t_index to end.
        let right_ctrl_pts = curve.coefficients[(t_index - p)..].to_vec();
        let right_weights = curve.weights[(t_index - p)..].to_vec();
        let right_knots = curve.knot_vector[t_index - p..].to_vec();
        let right_curve = NurbsCurve::try_new(right_ctrl_pts, right_weights, right_knots, p)?;

        Ok((left_curve, right_curve))
    }

    /// Generates the convex hull of the control polygon using the Quickhull algorithm.
    /// Returns a vector of TriangleFace representing the convex hull.
    pub fn control_polygon_hull(&self) -> AlgebraResult<ConvexHull>
    where
        T: Clone + Into<Point>,
    {
        // Convert control points to Points
        let points: Vec<Point> = self.coefficients.iter().map(|p| p.clone().into()).collect();

        // Use quickhull to generate the convex hull
        Ok(ConvexHull::try_new(points)?)
    }
}

/// A "slow" evaluation for a NURBS curve.
/// This computes the weighted sum of the underlying B-spline basis functions and then normalizes.
/// (It uses the BSplineCurve's basis evaluation for each control point.)
impl<T> NurbsCurve<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: std::ops::Div<EFloat64, Output = AlgebraResult<T>>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    fn insert_knot(&self, t: EFloat64) -> AlgebraResult<Self> {
        let k = match self.find_span(t) {
            Some(span) => span,
            None => return Err("Knot not found".into()),
        };
        let p = self.degree;
        let n = self.coefficients.len() - 1;

        let mut new_coefficients = Vec::with_capacity(self.coefficients.len() + 1);
        let mut new_weights = Vec::with_capacity(self.weights.len() + 1);
        let mut new_knot_vector = Vec::with_capacity(self.knot_vector.len() + 1);

        // Build new knot vector: copy knots up to k (inclusive), insert t, then copy the remaining knots.
        for i in 0..=k {
            new_knot_vector.push(self.knot_vector[i].clone());
        }
        new_knot_vector.push(t.clone());
        for i in (k + 1)..self.knot_vector.len() {
            new_knot_vector.push(self.knot_vector[i].clone());
        }

        // The new control points and weights:
        // 1. Copy control points and weights unaffected by the insertion.
        for i in 0..(k - p + 1) {
            new_coefficients.push(self.coefficients[i].clone());
            new_weights.push(self.weights[i].clone());
        }

        // 2. Recompute the control points and weights affected by the insertion.
        for i in (k - p + 1)..=k {
            // Compute alpha = (t - knot_vector[i]) / (knot_vector[i+p] - knot_vector[i])
            let alpha = ((t.clone() - self.knot_vector[i].clone())
                / (self.knot_vector[i + p].clone() - self.knot_vector[i].clone()))
            .unwrap_or(EFloat64::zero());

            // In homogeneous coordinates:
            // Q[i] = (1 - alpha) * Q[i-1] + alpha * Q[i]
            // where Q[i] = (w[i] * P[i], w[i])
            let w1 = self.weights[i - 1].clone();
            let w2 = self.weights[i].clone();
            let p1 = self.coefficients[i - 1].clone();
            let p2 = self.coefficients[i].clone();

            // New weight: w_new = (1 - alpha) * w1 + alpha * w2
            let new_weight =
                w1.clone() * (EFloat64::one() - alpha.clone()) + w2.clone() * alpha.clone();
            new_weights.push(new_weight.clone());

            // New control point: P_new = ((1 - alpha) * w1 * P1 + alpha * w2 * P2) / w_new
            let new_point = (p1 * w1 * (EFloat64::one() - alpha.clone()) + p2 * w2 * alpha)
                / new_weight.clone();
            new_coefficients.push(new_point?);
        }

        // 3. Copy the remaining control points and weights.
        for i in k..=n {
            new_coefficients.push(self.coefficients[i].clone());
            new_weights.push(self.weights[i].clone());
        }

        NurbsCurve::try_new(new_coefficients, new_weights, new_knot_vector, self.degree)
    }

    pub fn eval_slow(&self, t: EFloat64) -> AlgebraResult<T> {
        let mut numerator = T::zero();
        let mut denominator = EFloat64::zero();

        for i in 0..self.coefficients.len() {
            // Reuse the BSpline basis functions.
            let basis = BSplineCurve::<EFloat64>::try_new_from_basis(
                i,
                self.degree,
                self.knot_vector.clone(),
            )
            .unwrap();
            let n = basis.eval(t);
            let wn = self.weights[i] * n;
            numerator = numerator + self.coefficients[i].clone() * wn;
            denominator = denominator + wn;
        }
        if denominator == EFloat64::zero() {
            return Err("Division by zero".into());
        }
        numerator / denominator
    }
}

impl<T> Display for NurbsCurve<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NurbsCurve(")?;
        for coeff in self.coefficients.iter() {
            write!(f, "{}, ", coeff)?;
        }
        write!(f, ")")
    }
}
/// Helper struct for homogeneous coordinates.
#[derive(Clone)]
struct NurbHelperPoint<T> {
    point: T,
    weight: EFloat64,
}

impl<T> NurbHelperPoint<T>
where
    T: HasZero,
{
    pub fn zero() -> Self {
        Self {
            point: T::zero(),
            weight: EFloat64::zero(),
        }
    }
}

impl<T> std::ops::Add for NurbHelperPoint<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            point: self.point + other.point,
            weight: self.weight + other.weight,
        }
    }
}

impl<T> std::ops::Mul<EFloat64> for NurbHelperPoint<T>
where
    T: std::ops::Mul<EFloat64, Output = T>,
{
    type Output = Self;
    fn mul(self, scalar: EFloat64) -> Self {
        Self {
            point: self.point * scalar,
            weight: self.weight * scalar,
        }
    }
}

impl<T> std::ops::Mul<NurbHelperPoint<T>> for EFloat64
where
    T: std::ops::Mul<EFloat64, Output = T>,
{
    type Output = NurbHelperPoint<T>;
    fn mul(self, rhs: NurbHelperPoint<T>) -> NurbHelperPoint<T> {
        rhs * self
    }
}

/// Evaluate a NURBS curve using a rational de Boor algorithm.
/// This method first lifts the control points into homogeneous coordinates:
/// Qᵢ = (wᵢ * Pᵢ, wᵢ)
/// Then de Boor's algorithm is applied and the resulting point is projected back
/// (by dividing by its weight).
impl<T> MultiDimensionFunction<T> for NurbsCurve<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: std::ops::Div<EFloat64, Output = AlgebraResult<T>>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    fn eval(&self, t: EFloat64) -> T {
        // return self.eval_slow(t).unwrap_or(T::zero());

        let k = match self.find_span(t) {
            Some(span) => span,
            None => return T::zero(),
        };
        let p = self.degree;

        let mut d: Vec<NurbHelperPoint<T>> = Vec::with_capacity(p + 1);

        // Initialize homogeneous control points: Qᵢ = (wᵢ * Pᵢ, wᵢ)
        for j in 0..=p {
            if k + j < p || k + j - p >= self.coefficients.len() {
                d.push(NurbHelperPoint {
                    point: T::zero(),
                    weight: EFloat64::zero(),
                });
            } else {
                let idx = k + j - p;
                d.push(NurbHelperPoint {
                    point: self.coefficients[idx].clone() * self.weights[idx],
                    weight: self.weights[idx],
                });
            }
        }

        // Apply de Boor's algorithm in homogeneous coordinates.
        // The recurrence is:
        //   d[j] = (1 - α) * d[j-1] + α * d[j],
        // where α = (t - knot[start+j]) / (knot[j + k - r + 1] - knot[start+j])
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
        let dh = d[p].clone();
        if dh.weight == EFloat64::zero() {
            return T::zero();
        }
        (dh.point / dh.weight).unwrap_or(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efloat::EFloat64;

    fn to_efloat_vec(values: Vec<f64>) -> Vec<EFloat64> {
        values.into_iter().map(EFloat64::from).collect()
    }

    #[test]
    fn test_nurbs_values_equal() {
        // Create a NURBS curve with scalar control points.
        let coefficients = vec![
            EFloat64::from(5.0),
            EFloat64::from(1.0),
            EFloat64::from(3.0),
            EFloat64::from(2.0),
        ];

        // Use non-uniform weights for demonstration.
        let weights = to_efloat_vec(vec![1.0, 2.0, 1.0, 1.0]);

        // Strictly increasing knot vector.
        let knot_vector = to_efloat_vec(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);

        let nurbs = NurbsCurve::try_new(coefficients, weights, knot_vector, 3).unwrap();

        // Test at various parameter values.
        let test_params = to_efloat_vec(vec![1.5, 2.0, 2.5, 3.5, 4.5]);

        for t in test_params {
            let result_eval = nurbs.eval(t);
            let result_slow = nurbs.eval_slow(t);
            assert_eq!(result_eval, result_slow.unwrap());
        }
    }

    #[test]
    fn test_nurbs_knot_insertion() -> AlgebraResult<()> {
        // Create a NURBS curve with 2D points as coefficients
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

        // Use non-uniform weights for demonstration.
        let weights = to_efloat_vec(vec![1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0]);
        let degree = 3;

        // Strictly increasing knot vector
        let knot_vector = to_efloat_vec(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0,
        ]);

        let nurbs = NurbsCurve::try_new(coefficients, weights, knot_vector, degree).unwrap();
        println!("nurbs: {}", nurbs);

        // Choose a subdivision parameter in the valid domain.
        let t = EFloat64::from(2.5);
        // Subdivide the NURBS at t.
        let nurbs2 = nurbs.insert_knot(t.clone())?;

        // print
        println!("nurbs2: {}", nurbs2);

        for i in 0..=100 {
            let t = i as f64 / 100.0 * 7.0;
            let t = EFloat64::from(t);
            let result_eval = nurbs.eval(t.clone());
            let result2_eval = nurbs2.eval(t.clone());
            assert_eq!(result_eval, result2_eval);
        }
        Ok(())
    }

    #[test]
    fn test_nurbs_subdivide() -> AlgebraResult<()> {
        // Create a NURBS curve with scalar control points.
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

        // Use non-uniform weights for demonstration.
        let weights = to_efloat_vec(vec![1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0]);

        // Strictly increasing knot vector
        let knot_vector = to_efloat_vec(vec![
            0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0,
        ]);

        let nurbs = NurbsCurve::try_new(coefficients, weights, knot_vector, 3).unwrap();
        println!("Original NURBS: {}", nurbs);

        // Choose a subdivision parameter in the valid domain.
        let t = EFloat64::from(2.5);
        // Subdivide the NURBS at t.
        let (left, right) = nurbs.subdivide(t.clone())?;

        println!("Left segment: {}", left);
        println!("Right segment: {}", right);

        // Test the left segment (from 0 to t)
        for i in 0..100 {
            let param = i as f64 / 100.0 * 2.5;
            let param = EFloat64::from(param);
            let orig_val = nurbs.eval(param.clone());
            let left_val = left.eval(param.clone());
            assert_eq!(
                orig_val, left_val,
                "Left segment evaluation at t={} does not match the original curve",
                param
            );
        }

        // Test the right segment (from t to 5.0)
        for i in 0..100 {
            let param = 2.5 + i as f64 / 100.0 * 2.5;
            let param = EFloat64::from(param);
            let orig_val = nurbs.eval(param.clone());
            let right_val = right.eval(param.clone());
            assert_eq!(
                orig_val, right_val,
                "Right segment evaluation at t={} does not match the original curve",
                param
            );
        }

        // Test that the segments join correctly at the subdivision point
        let orig_val = nurbs.eval(t.clone());
        let left_val = left.eval(t.clone());
        let right_val = right.eval(t.clone());
        assert_eq!(
            orig_val, left_val,
            "Left segment evaluation at subdivision point t={} does not match the original curve",
            t
        );
        assert_eq!(
            orig_val, right_val,
            "Right segment evaluation at subdivision point t={} does not match the original curve",
            t
        );

        Ok(())
    }
}
