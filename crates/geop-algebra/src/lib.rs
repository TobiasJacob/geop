pub mod algebra_error;
pub mod bernstein_basis;
pub mod bernstein_polynomial;
pub mod efloat;
pub mod factorial;
pub mod monomial_polynom;

use monomial_polynom::MonomialPolynom;

pub trait HasZero {
    fn zero() -> Self;
}

pub trait ToMonomialPolynom {
    fn to_monomial_polynom(&self) -> MonomialPolynom;
}
