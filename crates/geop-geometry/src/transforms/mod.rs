use std::{fmt::Display, ops::Mul};

use crate::efloat::EFloat64;

use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub matrix: [[EFloat64; 4]; 4],
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{} ", self.matrix[i][j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut matrix = [[EFloat64::zero(); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    matrix[i][j] = matrix[i][j] + self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Transform { matrix }
    }
}

impl Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let other = [other.x, other.y, other.z, EFloat64::one()];
        let mut result = [EFloat64::zero(); 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i] = result[i] + self.matrix[i][j] * other[j];
            }
        }
        result[0] = (result[0] / result[3]).unwrap();
        result[1] = (result[1] / result[3]).unwrap();
        result[2] = (result[2] / result[3]).unwrap();
        Point::new(result[0], result[1], result[2])
    }
}

impl Mul<Option<Point>> for Transform {
    type Output = Option<Point>;

    fn mul(self, other: Option<Point>) -> Option<Point> {
        match other {
            Some(point) => Some(self * point),
            None => None,
        }
    }
}

impl Transform {
    pub fn from_translation(point: Point) -> Transform {
        let mut matrix = [[EFloat64::zero(); 4]; 4];
        matrix[0][0] = EFloat64::one();
        matrix[1][1] = EFloat64::one();
        matrix[2][2] = EFloat64::one();
        matrix[3][3] = EFloat64::one();
        matrix[0][3] = point.x;
        matrix[1][3] = point.y;
        matrix[2][3] = point.z;
        Transform { matrix }
    }

    pub fn from_euler_angles(roll: EFloat64, pitch: EFloat64, yaw: EFloat64) -> Transform {
        let mut matrix = [[EFloat64::zero(); 4]; 4];
        let (sin_x, cos_x) = (roll.sin(), roll.cos());
        let (sin_y, cos_y) = (pitch.sin(), pitch.cos());
        let (sin_z, cos_z) = (yaw.sin(), yaw.cos());
        matrix[0][0] = cos_y * cos_z;
        matrix[0][1] = -cos_y * sin_z;
        matrix[0][2] = sin_y;
        matrix[1][0] = sin_x * sin_y * cos_z + cos_x * sin_z;
        matrix[1][1] = -sin_x * sin_y * sin_z + cos_x * cos_z;
        matrix[1][2] = -sin_x * cos_y;
        matrix[2][0] = -cos_x * sin_y * cos_z + sin_x * sin_z;
        matrix[2][1] = cos_x * sin_y * sin_z + sin_x * cos_z;
        matrix[2][2] = cos_x * cos_y;
        matrix[3][3] = EFloat64::one();
        println!("{}", Transform { matrix });
        Transform { matrix }
    }

    pub fn from_scale(scale: Point) -> Transform {
        let mut matrix = [[EFloat64::zero(); 4]; 4];
        matrix[0][0] = scale.x;
        matrix[1][1] = scale.y;
        matrix[2][2] = scale.z;
        matrix[3][3] = EFloat64::one();
        Transform { matrix }
    }

    pub fn uniform_scale_factor(&self) -> EFloat64 {
        let scale_x = self.matrix[0][0] + self.matrix[0][1] + self.matrix[0][2];
        let scale_y = self.matrix[1][0] + self.matrix[1][1] + self.matrix[1][2];
        let scale_z = self.matrix[2][0] + self.matrix[2][1] + self.matrix[2][2];
        assert!(
            (scale_x.abs() - scale_y.abs()) == 0.0,
            "Scale must be uniform"
        );
        assert!(
            (scale_x.abs() - scale_z.abs()) == 0.0,
            "Scale must be uniform"
        );
        return scale_x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let t1 = Transform::from_translation(Point::from_f64(1.0, 2.0, 3.0));
        let t2 = Transform::from_translation(Point::from_f64(4.0, 5.0, 6.0));
        let t3 = t1 * t2;
        assert_eq!(
            t3 * Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn test_rotation() {
        let t1 =
            Transform::from_euler_angles(EFloat64::zero(), EFloat64::zero(), EFloat64::half_pi());
        println!("{}", t1);
        let t2 =
            Transform::from_euler_angles(EFloat64::half_pi(), EFloat64::zero(), EFloat64::zero());
        let t3 = t2 * t1.clone();
        assert_eq!(
            t1 * Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0)
        );
        assert_eq!(
            t3 * Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0)
        );
    }
}
