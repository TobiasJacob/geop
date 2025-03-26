use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    bspline_curve::BSplineCurve,
    efloat::EFloat64,
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
}

/// A “slow” evaluation for a NURBS curve.
/// This computes the weighted sum of the underlying B-spline basis functions and then normalizes.
/// (It uses the BSplineCurve’s basis evaluation for each control point.)
impl<T> NurbsCurve<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: std::ops::Div<EFloat64, Output = AlgebraResult<T>>,
    T: HasZero,
    T: ToMonomialPolynom,
{
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
/// Then de Boor’s algorithm is applied and the resulting point is projected back
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
}
