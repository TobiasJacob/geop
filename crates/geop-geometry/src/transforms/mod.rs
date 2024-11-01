use std::ops::Mul;

use crate::{point::Point, EQ_THRESHOLD};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub matrix: [[f64; 4]; 4],
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut matrix = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = 0.0;
                for k in 0..4 {
                    matrix[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Transform { matrix }
    }
}

impl Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let other = [other.x, other.y, other.z, 1.0];
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = 0.0;
            for j in 0..4 {
                result[i] += self.matrix[i][j] * other[j];
            }
        }
        result[0] /= result[3];
        result[1] /= result[3];
        result[2] /= result[3];
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
        let mut matrix = [[0.0; 4]; 4];
        matrix[0][0] = 1.0;
        matrix[1][1] = 1.0;
        matrix[2][2] = 1.0;
        matrix[3][3] = 1.0;
        matrix[0][3] = point.x;
        matrix[1][3] = point.y;
        matrix[2][3] = point.z;
        Transform { matrix }
    }

    pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Transform {
        let mut matrix = [[0.0; 4]; 4];
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
        matrix[3][3] = 1.0;
        Transform { matrix }
    }

    pub fn from_scale(scale: Point) -> Transform {
        let mut matrix = [[0.0; 4]; 4];
        matrix[0][0] = scale.x;
        matrix[1][1] = scale.y;
        matrix[2][2] = scale.z;
        matrix[3][3] = 1.0;
        Transform { matrix }
    }

    pub fn uniform_scale_factor(&self) -> f64 {
        let scale_x = self.matrix[0][0] + self.matrix[0][1] + self.matrix[0][2];
        let scale_y = self.matrix[1][0] + self.matrix[1][1] + self.matrix[1][2];
        let scale_z = self.matrix[2][0] + self.matrix[2][1] + self.matrix[2][2];
        assert!(
            (scale_x - scale_y).abs() < EQ_THRESHOLD,
            "Scale must be uniform"
        );
        assert!(
            (scale_x - scale_z).abs() < EQ_THRESHOLD,
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
        let t1 = Transform::from_translation(Point::new(1.0, 2.0, 3.0));
        let t2 = Transform::from_translation(Point::new(4.0, 5.0, 6.0));
        let t3 = t1 * t2;
        assert_eq!(t3 * Point::new(0.0, 0.0, 0.0), Point::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_rotation() {
        let t1 = Transform::from_euler_angles(0.0, 0.0, std::f64::consts::PI / 2.0);
        let t2 = Transform::from_euler_angles(std::f64::consts::PI / 2.0, 0.0, 0.0);
        let t3 = t2 * t1.clone();
        assert_eq!(t1 * Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0));
        assert_eq!(t3 * Point::new(1.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0));
    }
}
