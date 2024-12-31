use crate::{
    algebra_error::AlgebraResult, bspline_basis::BSplineBasis, efloat::EFloat64, HasZero,
    MultiDimensionFunction, OneDimensionFunction, ToMonomialPolynom,
};

pub struct BSplineCurve<T> {
    pub coefficients: Vec<T>,
    knot_vector: Vec<EFloat64>,
}

impl<T> BSplineCurve<T> {
    pub fn try_new(coefficients: Vec<T>, knot_vector: Vec<EFloat64>) -> AlgebraResult<Self> {
        BSplineBasis::new(0, coefficients.len(), knot_vector.clone())?;

        Ok(Self {
            coefficients,
            knot_vector,
        })
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
