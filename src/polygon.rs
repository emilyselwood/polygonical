use float_cmp::approx_eq;

use crate::{point::Point, boundingbox::BoundingBox};
use std::f64::consts::PI;


pub struct Polygon {
    pub points : Vec<Point>,
    pub bounds : BoundingBox,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        let bounds = BoundingBox::from_points(&points);
        return Polygon { 
            points, 
            bounds,
        }
    }

    // TODO: circles
    // TODO: rectangle


    /*
    contains returns true if the point p is inside of this polygon
    */
    pub fn contains(self, p: Point) -> bool {
        // fast path check with the bounding box first, if its outside that then it can never be inside the polygon.
        if !self.bounds.contains(p) {
            return false
        }

        let mut total : f64 = 0.0;

        for point in self.points {
            total = total + p.angle_to(point);
        }

        approx_eq!(f64, total.abs(), 2.0*PI, ulps = 2)
    }

}



#[cfg(test)]
mod tests {
    use crate::boundingbox::BoundingBox;
    use crate::point::Point;

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
}