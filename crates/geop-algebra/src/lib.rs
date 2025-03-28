pub mod algebra_error;
pub mod bernstein_polynomial;
pub mod bspline_curve;
pub mod convex_hull;
pub mod efloat;
pub mod factorial;
pub mod line;
pub mod monomial_polynom;
pub mod nurbs_curve;
pub mod point;
pub mod triangle;

use efloat::EFloat64;
use monomial_polynom::MonomialPolynom;

pub trait HasZero {
    fn zero() -> Self;
}

pub trait ToMonomialPolynom {
    fn to_monomial_polynom(&self) -> MonomialPolynom;
}

pub trait OneDimensionFunction {
    fn eval(&self, t: EFloat64) -> EFloat64;
}

pub trait MultiDimensionFunction<T> {
    fn eval(&self, t: EFloat64) -> T;
}
