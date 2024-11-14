use crate::efloat::EFloat64;

pub struct BernsteinPolynomial {
    coefficients: Vec<EFloat64>,
}

impl BernsteinPolynomial {
    pub fn new(coefficients: Vec<EFloat64>) -> Self {
        Self { coefficients }
    }

    pub fn eval(&self, t: EFloat64) -> EFloat64 {
        todo!()
    }
}
