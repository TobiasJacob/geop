use crate::{curves::line::Line, points::point::Point};

#[derive(Debug)]
pub enum LineLineIntersection {
    Line(Line),
    Point(Point),
    None,
}

pub fn line_line_intersection(a: &Line, b: &Line) -> LineLineIntersection {
    let v1 = a.direction;
    let v2 = b.direction;
    let p1 = a.basis;
    let p2 = b.basis;

    if v1.is_parallel(v2) {
        if (p1 - p2).is_parallel(v1) {
            return LineLineIntersection::Line(Line::new(p1, v1));
        } else {
            return LineLineIntersection::None;
        }
    }

    let cross_product = v1.cross(v2);
    let t = (p2 - p1).cross(v2).dot(cross_product) / cross_product.norm_sq();
    let p = p1 + v1 * t;

    if (p - p1).is_parallel(v1) && (p - p2).is_parallel(v2) {
        return LineLineIntersection::Point(p);
    } else {
        return LineLineIntersection::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_line_intersection() {
        let l1 = Line::new(Point::new(-2.0, 1.0, 4.0), Point::new(1.0, 0.0, 0.0));
        let l2 = Line::new(Point::new(-2.0, 1.0, 4.0), Point::new(0.0, 1.0, 0.0));
        let i = line_line_intersection(&l1, &l2);
        match i {
            LineLineIntersection::Point(p) => {
                assert_eq!(p, Point::new(-2.0, 1.0, 4.0));
            }
            _ => panic!("Expected point intersection"),
        }

        let l3 = Line::new(Point::new(0.0, 1.0, 4.0), Point::new(2.0, 0.0, 0.0));
        match line_line_intersection(&l1, &l3) {
            LineLineIntersection::Line(l) => {
                assert_eq!(
                    l,
                    Line::new(Point::new(-2.0, 1.0, 4.0), Point::new(1.0, 0.0, 0.0))
                );
            }
            _ => panic!("Expected line intersection"),
        }

        let l4 = Line::new(Point::new(0.0, -1.0, 4.0), Point::new(0.0, 1.0, 0.0));
        match line_line_intersection(&l1, &l4) {
            LineLineIntersection::Point(p) => {
                assert_eq!(p, Point::new(0.0, 1.0, 4.0));
            }
            _ => panic!("Expected point intersection"),
        }

        let l5 = Line::new(Point::new(0.0, 1.0, 3.0), Point::new(0.0, 1.0, 0.0));
        match line_line_intersection(&l1, &l5) {
            LineLineIntersection::None => {}
            _ => panic!("Expected no intersection"),
        }
    }
}
