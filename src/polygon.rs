use float_cmp::approx_eq;

use crate::{boundingbox::BoundingBox, geom, point::Point};
use std::{
    f64::consts::PI,
    fmt::{self, Display},
    iter::zip,
};

#[allow(clippy::len_without_is_empty)] // a polygon can never be empty so an is_empty function would always return false.
#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    pub bounds: BoundingBox,
}

impl Polygon {
    /// Create a new polygon.
    ///
    /// The vector of points must contain at least 3 elements or this will panic.
    pub fn new(points: Vec<Point>) -> Self {
        if points.len() < 3 {
            panic!(
                "Trying to create a polygon with {} points. You need at least 3",
                points.len()
            )
        }

        let bounds = BoundingBox::from_points(&points);
        Polygon { points, bounds }
    }

    // TODO: circles
    // TODO: rectangle

    /// Return the number of points in this polygon
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Return the two points describing a side of this polygon. Indexing from zero.
    pub fn get_side(&self, i: usize) -> (Point, Point) {
        let p1 = self.points[i];
        // handle that the polygon wraps around back to the start.
        let p2: Point = if i + 1 >= self.points.len() {
            self.points[0]
        } else {
            self.points[i + 1]
        };

        (p1, p2)
    }

    /// Return a vector of point pairs for every side of this polygon, in order.
    pub fn sides(&self) -> Vec<(Point, Point)> {
        let mut result = Vec::new();

        for i in 0..self.len() {
            result.push(self.get_side(i));
        }

        result
    }

    /// Do any of the lines of this polygon cross over any other lines?
    pub fn is_self_intersecting(&self) -> bool {
        for i in 0..self.points.len() {
            let (p1, p2) = self.get_side(i);
            for j in i + 1..self.points.len() {
                let (p3, p4) = self.get_side(j);
                // if the two lines share a point then skip this as yes they intersect according to the
                // geom function but not as far as this is concerned.
                if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 {
                    continue;
                }
                if geom::lines_intersect(p1, p2, p3, p4) {
                    return true;
                }
            }
        }

        false
    }

    /// Return the area of this polygon
    /// Note: This will panic if the polygon is self intersecting.
    pub fn area(&self) -> f64 {
        if self.is_self_intersecting() {
            panic!("Can not calculate the area of a self intersecting polygon")
        }

        let sides = self.sides();
        let triangle_sum = sides
            .iter()
            .map(|s| geom::area_of_triangle(Point::zero(), s.0, s.1))
            .sum();

        triangle_sum
    }

    /// Return the point average of this polygon giving a possible centre
    pub fn center(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;

        for p in self.points.iter() {
            x += p.x;
            y += p.y;
        }
        let len = self.len() as f64;

        Point::new(x / len, y / len)
    }

    /// Contains returns true if the point p is inside of this polygon
    pub fn contains(&self, p: Point) -> bool {
        // fast path check with the bounding box first, if its outside that then it can never be inside the polygon.
        if !self.bounds.contains(p) {
            return false;
        }

        let total: f64 = self.points.iter().map(|point| p.angle_to(point)).sum();

        approx_eq!(f64, total.abs(), 2.0 * PI, ulps = 2)
    }

    /// Move this polygon by point p
    pub fn translate(&self, p: Point) -> Polygon {
        let points = self.points.iter().map(|point| point.translate(p)).collect();
        Polygon::new(points)
    }

    /// Rotate a polygon counter clockwise around its center point by angle radians
    pub fn rotate_around_center(&self, angle: f64) -> Polygon {
        let center = self.center();
        let center_inv = center.invert();

        let new_points = self
            .points
            .iter()
            .map(|p| p.translate(center_inv).rotate(angle).translate(center))
            .collect();

        Polygon::new(new_points)
    }

    /// Rotate the entire polygon counter clockwise around the origin by angle radians
    pub fn rotate_around_origin(&self, angle: f64) -> Polygon {
        let new_points = self.points.iter().map(|p| p.rotate(angle)).collect();

        Polygon::new(new_points)
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        // if all the points in both polygons are equal then they are equal
        if other.len() != self.len() {
            return false;
        }

        for (a, b) in zip(self.points.iter(), other.points.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl Display for Polygon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Poly(")?;

        let mut first = true;
        for p in self.points.iter() {
            if !first {
                write!(formatter, ", ")?;
            } else {
                first = false;
            }
            p.fmt(formatter)?;
        }

        write!(formatter, ")")
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::boundingbox::BoundingBox;
    use crate::point::Point;

    use super::Polygon;

    #[test]
    fn does_not_contain() {
        let bbox = BoundingBox::new(Point::zero(), Point::new(2.0, 2.0)).to_polygon();
        assert!(!bbox.contains(Point::new(5.0, 1.0)))
    }

    #[test]
    fn does_contain() {
        let bbox = BoundingBox::new(Point::zero(), Point::new(2.0, 2.0)).to_polygon();
        assert!(!bbox.contains(Point::new(1.0, 1.0)))
    }

    #[test]
    fn is_self_intersecting() {
        let poly = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
        ]);

        assert!(poly.is_self_intersecting())
    }

    #[test]
    fn is_not_self_intersecting() {
        let poly = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);

        assert!(!poly.is_self_intersecting())
    }

    #[test]
    fn sides_square() {
        let poly = Polygon::new(vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
        ]);

        let result = poly.sides();

        let expected = vec![
            (Point::new(1.0, 0.0), Point::new(0.0, 0.0)),
            (Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
            (Point::new(0.0, 1.0), Point::new(1.0, 1.0)),
            (Point::new(1.0, 1.0), Point::new(1.0, 0.0)),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn check_area() {
        let poly = Polygon::new(vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
        ]);

        let result = poly.area();

        println!("result area: {} expected area: 1", result);
        assert!(approx_eq!(f64, result, 1.0, ulps = 2))
    }

    #[test]
    fn rotate_square() {
        let poly = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);

        let result = poly.rotate_around_center(90.0_f64.to_radians());

        println!("result: {}", result);

        // area should be the same after rotating the polygon
        assert!(approx_eq!(f64, result.area(), poly.area(), ulps = 2));

        let expected = Polygon::new(vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn rotate_square_around_origin() {
        let poly = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);

        let result = poly.rotate_around_origin(90.0_f64.to_radians());
        let expected = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(-1.0, 0.0),
            Point::new(-1.0, 1.0),
            Point::new(0.0, 1.0),
        ]);

        assert_eq!(result, expected);

        assert_eq!(result.area(), poly.area());
    }
}
