use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
};

#[derive(Debug, Clone)]
pub struct MonomialPolynom {
    pub monomials: Vec<EFloat64>,
}

impl MonomialPolynom {
    pub fn new(monomials: Vec<EFloat64>) -> Self {
        // Strip leading zeros
        let mut monomials = monomials;
        while monomials.len() > 1 && *monomials.last().unwrap() == 0.0 {
            monomials.pop();
        }
        Self { monomials }
    }

    pub fn from_factor(factor: EFloat64) -> Self {
        Self::new(vec![factor])
    }

    pub fn eval(&self, x: EFloat64) -> EFloat64 {
        let mut result = EFloat64::zero();
        for i in 0..self.monomials.len() {
            result = result + self.monomials[i] * x.powi(i as i32);
        }
        result
    }

    pub fn is_zero(&self) -> bool {
        self.monomials.len() == 0
    }
}

impl std::ops::Add for &MonomialPolynom {
    type Output = MonomialPolynom;

    fn add(self, other: &MonomialPolynom) -> MonomialPolynom {
        let mut result = vec![EFloat64::zero(); self.monomials.len().max(other.monomials.len())];
        for i in 0..result.len() {
            result[i] = *self.monomials.get(i).unwrap_or(&EFloat64::zero())
                + *other.monomials.get(i).unwrap_or(&EFloat64::zero());
        }
        MonomialPolynom::new(result)
    }
}

impl std::ops::Mul for &MonomialPolynom {
    type Output = MonomialPolynom;

    fn mul(self, other: &MonomialPolynom) -> MonomialPolynom {
        let mut result = vec![EFloat64::zero(); self.monomials.len() + other.monomials.len() - 1];
        for i in 0..self.monomials.len() {
            for j in 0..other.monomials.len() {
                result[i + j] = result[i + j] + self.monomials[i] * other.monomials[j];
            }
        }
        MonomialPolynom::new(result)
    }
}

impl std::ops::Sub for &MonomialPolynom {
    type Output = MonomialPolynom;

    fn sub(self, other: &MonomialPolynom) -> MonomialPolynom {
        let mut result = vec![EFloat64::zero(); self.monomials.len().max(other.monomials.len())];
        for i in 0..result.len() {
            result[i] = *self.monomials.get(i).unwrap_or(&EFloat64::zero())
                - *other.monomials.get(i).unwrap_or(&EFloat64::zero());
        }
        MonomialPolynom::new(result)
    }
}

impl std::ops::Neg for &MonomialPolynom {
    type Output = MonomialPolynom;

    fn neg(self) -> MonomialPolynom {
        let mut result = vec![EFloat64::zero(); self.monomials.len()];
        for i in 0..result.len() {
            result[i] = -self.monomials[i];
        }
        MonomialPolynom::new(result)
    }
}

// Division with remainder
impl std::ops::Div for MonomialPolynom {
    type Output = AlgebraResult<(Self, Self)>;

    fn div(self, other: Self) -> AlgebraResult<(Self, Self)> {
        if other.monomials.len() == 0 {
            return Err(AlgebraError::new("Division by zero".to_string()).into());
        }

        let mut result = Vec::with_capacity(self.monomials.len() - other.monomials.len() + 1);
        let mut remainder = self.clone();
        while remainder.monomials.len() >= other.monomials.len() {
            let factor = (*remainder.monomials.last().unwrap() / *other.monomials.last().unwrap())?;
            let subtractor = &other * &MonomialPolynom::from_factor(factor);
            remainder = &remainder - &subtractor;
            result.push(factor);
        }
        result.reverse();
        Ok((MonomialPolynom::new(result), remainder))
    }
}
