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
        while monomials.len() > 0 && *monomials.last().unwrap() == 0.0 {
            monomials.pop();
        }
        Self { monomials }
    }

    pub fn from_factor(factor: EFloat64) -> Self {
        Self::new(vec![factor])
    }

    pub fn from_factor_and_power(factor: EFloat64, power: usize) -> Self {
        let mut monomials = vec![EFloat64::zero(); power];
        monomials.push(factor);
        Self::new(monomials)
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

    pub fn pow(&self, power: usize) -> Self {
        if power == 0 {
            return Self::from_factor(EFloat64::one());
        }
        let mut result = self.clone();
        for _ in 1..power {
            result = &result * self;
        }
        result
    }
}

impl PartialEq for MonomialPolynom {
    fn eq(&self, other: &Self) -> bool {
        if self.monomials.len() != other.monomials.len() {
            return false;
        }
        for i in 0..self.monomials.len() {
            if self.monomials[i] != other.monomials[i] {
                return false;
            }
        }
        true
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
        let mut result = vec![EFloat64::zero(); self.monomials.len() + other.monomials.len()];
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
impl std::ops::Div for &MonomialPolynom {
    type Output = AlgebraResult<(MonomialPolynom, MonomialPolynom)>;

    fn div(self, other: &MonomialPolynom) -> AlgebraResult<(MonomialPolynom, MonomialPolynom)> {
        if other.monomials.len() == 0 {
            return Err(AlgebraError::new("Division by zero".to_string()).into());
        }

        let mut result = Vec::with_capacity(self.monomials.len() - other.monomials.len());
        let mut remainder = self.clone();
        while remainder.monomials.len() >= other.monomials.len() {
            let factor = (*remainder.monomials.last().unwrap() / *other.monomials.last().unwrap())?;
            let subtractor = other
                * &MonomialPolynom::from_factor_and_power(
                    factor,
                    remainder.monomials.len() - other.monomials.len(),
                );
            remainder = &remainder - &subtractor;
            result.push(factor);
        }
        result.reverse();
        Ok((MonomialPolynom::new(result), remainder))
    }
}

impl std::fmt::Display for &MonomialPolynom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        write!(f, "(")?;
        for i in (0..self.monomials.len()).rev() {
            if *self.monomials.get(i).unwrap() != 0.0 {
                if !first {
                    write!(f, " + ")?;
                }
                write!(f, "{}x^{}", self.monomials[i], i)?;
                first = false;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

// Mul with Efloat
impl std::ops::Mul<EFloat64> for &MonomialPolynom {
    type Output = MonomialPolynom;

    fn mul(self, other: EFloat64) -> MonomialPolynom {
        let mut result = vec![EFloat64::zero(); self.monomials.len()];
        for i in 0..result.len() {
            result[i] = self.monomials[i] * other;
        }
        MonomialPolynom::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efloat::EFloat64;

    #[test]
    fn test_monomial_polynom() {
        let p1 = &MonomialPolynom::new(vec![
            EFloat64::from(4.0),
            EFloat64::from(5.0),
            EFloat64::from(2.0),
        ]);

        let p2 = &MonomialPolynom::new(vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(3.0),
        ]);

        let p3 = p1 * p2;
        let (result, remainder) = (&p3 / p2).unwrap();
        assert!(remainder.is_zero());
        assert_eq!(result, *p1);
    }
}
