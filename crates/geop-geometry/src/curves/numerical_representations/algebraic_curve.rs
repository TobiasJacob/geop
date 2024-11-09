pub struct PolynomialCurve {
    pub coefficients: Vec<f64>,
}

pub struct RationalCurve {
    pub numerator: Polynomial,
    pub denominator: Polynomial,
}

trait AsPolynomialCurve {
    pub fn as_polynomial(&self) -> Polynomial;
}

trait AsRationalCurve {
    pub fn as_rational(&self) -> RationalPolynomial;
}
