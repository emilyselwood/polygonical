use std::fmt;

use crate::{point::Point, polygon::Polygon};

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    a: Point,
    b: Point,
}

impl BoundingBox {
    pub fn new(a: Point, b: Point) -> Self {
        BoundingBox { a, b }
    }

    pub fn from_points(points: &Vec<Point>) -> Self {
        let mut min = Point::new_max();
        let mut max = Point::new_min();

        for p in points {
            min = p.min(min);
            max = p.max(max);
        }

        BoundingBox { a: min, b: max }
    }

    pub fn contains(&self, p: Point) -> bool {
        self.a.x <= p.x && self.b.x >= p.x && self.a.y <= p.y && self.b.y >= p.y
    }

    pub fn to_polygon(&self) -> Polygon {
        let points = vec![
            Point::new(self.a.x, self.a.y),
            Point::new(self.a.x, self.b.y),
            Point::new(self.b.x, self.b.y),
            Point::new(self.b.x, self.a.y),
        ];
        Polygon::new(points)
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        other.contains(Point::new(self.a.x, self.a.y))
            || other.contains(Point::new(self.b.x, self.a.y))
            || other.contains(Point::new(self.a.x, self.b.y))
            || other.contains(Point::new(self.b.x, self.b.y)) 
            || self.contains(Point::new(other.a.x, other.a.y))
            || self.contains(Point::new(other.b.x, other.a.y))
            || self.contains(Point::new(other.a.x, other.b.y))
            || self.contains(Point::new(other.b.x, other.b.y)) 
            || (
                other.a.x >= self.a.x && other.b.x <= self.b.x && self.a.y >= other.a.y && self.b.y <= other.b.y
            )
            || (
                other.a.y >= self.a.y && other.b.y <= self.b.y && self.a.x >= other.a.x && self.b.x <= other.b.x
            )
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "BoundingBox({}, {})", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::boundingbox::BoundingBox;
    use crate::point::Point;

    #[test]
    fn does_not_contain() {
        let bbox = BoundingBox::new(Point::zero(), Point::new(2.0, 2.0));
        assert!(!bbox.contains(Point::new(5.0, 1.0)))
    }

    #[test]
    fn does_contain() {
        let bbox = BoundingBox::new(Point::zero(), Point::new(2.0, 2.0));
        assert!(bbox.contains(Point::new(1.0, 1.0)))
    }

    macro_rules! intersection_tests {
        ($($name:ident: $boxa:expr, $boxb:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let a = BoundingBox::new($boxa.0, $boxa.1);
                    let b = BoundingBox::new($boxb.0, $boxb.1);

                    assert_eq!(a.intersects(&b), $expected);
                }
            )*
        };
    }

    intersection_tests!(
        non_intersecting: (Point::new(0.0, 0.0), Point::new(1.0, 1.0)), (Point::new(2.0, 2.0), Point::new(3.0, 3.0)), false,
        cross_intersecting: (Point::new(1.0, 0.0), Point::new(2.0, 3.0)), (Point::new(0.0, 1.0), Point::new(3.0, 2.0)), true,
        entirely_contained: (Point::new(0.0, 0.0), Point::new(3.0, 3.0)), (Point::new(1.0, 1.0), Point::new(2.0, 2.0)), true,
        entirely_contains: (Point::new(1.0, 1.0), Point::new(2.0, 2.0)), (Point::new(0.0, 0.0), Point::new(3.0, 3.0)), true,
        just_corner: (Point::new(0.0, 0.0), Point::new(1.0, 1.0)), (Point::new(1.0, 1.0), Point::new(2.0, 2.0)), true,
    );

}
