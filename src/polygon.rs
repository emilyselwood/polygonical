use float_cmp::approx_eq;

use crate::{boundingbox::BoundingBox, geom, point::Point};
use std::f64::consts::PI;

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
        return Polygon { points, bounds };
    }

    // TODO: circles
    // TODO: rectangle

    /// return the number of points in this polygon
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// get_side returns the two points describing a side of this polygon. Indexing from zero.
    pub fn get_side(&self, i: usize) -> (Point, Point) {
        let p1 = self.points[i];
        let p2: Point;
        // handle that the polygon wraps around back to the start.
        if i + 1 >= self.points.len() {
            p2 = self.points[0];
        } else {
            p2 = self.points[i + 1];
        }
        (p1, p2)
    }

    /// get a vector of point pairs for every side of this polygon, in order.
    pub fn sides(&self) -> Vec<(Point, Point)> {
        let mut result = Vec::new();

        for i in 0 .. self.len() {
            result.push(self.get_side(i));
        }

        result
    }

    /// Do any of the lines of this polygon cross over themselves?
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


    /// return the area of this polygon
    pub fn area(&self) -> f64 {
        let mut triangle_sum = 0.0;
        let sides = self.sides();
        for i in 0 .. sides.len()-1 {
            triangle_sum = triangle_sum + geom::area_of_triangle(Point::zero(), sides[i].0, sides[i].1)
        }
        
        triangle_sum
    }

    /// contains returns true if the point p is inside of this polygon
    pub fn contains(&self, p: Point) -> bool {
        // fast path check with the bounding box first, if its outside that then it can never be inside the polygon.
        if !self.bounds.contains(p) {
            return false;
        }

        let mut total: f64 = 0.0;

        for point in self.points.iter() {
            total = total + p.angle_to(point);
        }

        approx_eq!(f64, total.abs(), 2.0 * PI, ulps = 2)
    }
}

#[cfg(test)]
mod tests {
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
}
