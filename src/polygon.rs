use float_cmp::approx_eq;

use crate::{boundingbox::BoundingBox, geom, point::Point};
use std::{
    fmt::{self, Display},
    iter::zip,
    mem,
};

/// Polygon describes a the points around the edge of a shape. It can only contain and single path, no holes
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
        self.sides_from(0)
    }

    fn sides_from(&self, start: usize) -> Vec<(Point, Point)> {
        let mut result = Vec::new();

        // include the first half of the list
        for i in start..self.len() {
            result.push(self.get_side(i));
        }

        // now create the other bit.
        for i in 0..start {
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

        // work out the sum of the angles between adjacent points and the point we are checking.
        // if the sum is equal to 360 degrees then we are inside the polygon.
        let mut total = 0.0;

        for i in 0..self.points.len() {
            let (p1, p2) = self.get_side(i);
            let angle_a = p.angle_to(&p2);
            let angle_b = p.angle_to(&p1);

            // handle rolling around over the 360/0 degree line reasonably
            let result = if angle_a > angle_b {
                -((360.0_f64.to_radians() - angle_a) + angle_b)
            } else {
                angle_a - angle_b
            };

            total += result;
        }
        approx_eq!(f64, total.abs(), 360.0_f64.to_radians(), ulps = 2)
    }

    /// Returns true if any part of the other polygon overlaps this one.
    /// Entirely containing other or being contained by other counts here.
    /// Note: this has worst case runtime on two polygons that don't intersect but their bounding boxes do.
    /// O(n^2) where n is the sum of the number of sides in the two polygons
    pub fn intersects(&self, other: &Polygon) -> bool {
        // first check if the bounding boxes intersect as a quicker check
        if !self.bounds.intersects(&other.bounds) {
            return false;
        }

        // if any other sides intersect then the two polygons intersect
        // This also checks any of the points being the same due to the way the lines_intersect algorithm works
        let self_sides = self.sides();
        let other_sides = other.sides();

        for self_side in self_sides.iter() {
            for other_side in other_sides.iter() {
                if geom::lines_intersect(self_side.0, self_side.1, other_side.0, other_side.1) {
                    return true;
                }
            }
        }

        // If that wasn't true check if the first point of the other is inside this polygon.
        // the only way this could be true is if all the points are inside so we only need to check the first one
        if self.contains(other.points[0]) {
            return true;
        }

        // Also possible that the other one entirely contains this one
        if other.contains(self.points[0]) {
            return true;
        }

        // now we know that other does not intersect with this polygon
        false
    }

    /// Move this polygon by point p
    pub fn translate(&self, p: Point) -> Polygon {
        let points = self
            .points
            .iter()
            .map(|point| point.translate(&p))
            .collect();
        Polygon::new(points)
    }

    /// Rotate a polygon counter clockwise around its center point by angle radians
    pub fn rotate_around_center(&self, angle: f64) -> Polygon {
        let center = self.center();
        let center_inv = center.invert();

        let new_points = self
            .points
            .iter()
            .map(|p| p.translate(&center_inv).rotate(angle).translate(&center))
            .collect();

        Polygon::new(new_points)
    }

    /// Rotate the entire polygon counter clockwise around the origin by angle radians
    pub fn rotate_around_origin(&self, angle: f64) -> Polygon {
        let new_points = self.points.iter().map(|p| p.rotate(angle)).collect();

        Polygon::new(new_points)
    }

    /// Create a new polygon that is the union of this polygon and the other polygon provided.
    pub fn union(&self, other: &Polygon) -> Polygon {
        let mut result_points = Vec::new();
        result_points.push(self.points[0]);
        let mut current = self;
        let mut not_current = other;

        let mut current_index = 0;
        let mut other_index = 0;
        while current_index < current.len() {
            // get a side

            let current_side = current.get_side(current_index);
            // look for an intersecting side in the other one.
            let not_current_sides = not_current.sides_from(other_index);
            let intersects_with = geom::line_intersects_others(current_side, &not_current_sides);
            if let Some(oi) = intersects_with {
                let other_line = not_current_sides[oi];

                // Find the point of intersection (we can be pretty sure this intersects as we checked just now)
                let point = geom::point_of_intersection(
                    current_side.0,
                    current_side.1,
                    other_line.0,
                    other_line.1,
                )
                .unwrap();

                // add that point to the list
                result_points.push(point);
                // add the end of the intersecting line to the list, a two straight lines cant intersect twice.
                // At least not in this simple flat plain universe.
                result_points.push(other_line.1);

                // swap current and other
                mem::swap(&mut current, &mut not_current);

                // set other_index to current_index, don't add one because this might cross back over this line again
                other_index = current_index;
                // set current_index to intersects_with
                let mut target_index = other_index + oi;
                if target_index > not_current.len() {
                    target_index -= not_current.len();
                }
                current_index = target_index;
            } else {
                // Nothing intersects with this side so we can add the new end to the result list.
                result_points.push(current_side.1);
                current_index += 1;
            }
        }

        Polygon::new(result_points)
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

    use crate::{point::Point, tests::assert_f64};

    use super::Polygon;

    macro_rules! contains_tests {
        ($($name:ident: $poly_points:expr, $test_point:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let poly = Polygon::new($poly_points);
                    assert_eq!(poly.contains($test_point), $expected);
                }
            )*
        };
    }

    contains_tests!(
        not_in:
            vec![
                Point::zero(),
                Point::new(0.0, 2.0),
                Point::new(2.0, 2.0),
                Point::new(2.0, 0.0)
            ],
        Point::new(5.0, 1.0),
        false,
        inside:
            vec![
                Point::zero(),
                Point::new(0.0, 2.0),
                Point::new(2.0, 2.0),
                Point::new(2.0, 0.0)
            ],
        Point::new(1.0, 1.0),
        true,
    );

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

        assert_f64!(result, 1.0);
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

        // area should be the same after rotating the polygon
        assert_f64!(result.area(), poly.area());

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

        assert_f64!(result.area(), poly.area());
    }

    macro_rules! intersection_tests {
        ($($name:ident: $points_a:expr, $points_b:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let a = Polygon::new($points_a);
                    let b = Polygon::new($points_b);

                    assert_eq!(a.intersects(&b), $expected);
                }
            )*
        };
    }

    intersection_tests!(
        non_intersecting:
            vec![
                Point::new(0.0, 0.0),
                Point::new(1.0, 1.0),
                Point::new(1.0, 0.0)
            ],
        vec![
            Point::new(2.0, 2.0),
            Point::new(3.0, 3.0),
            Point::new(3.0, 2.0)
        ],
        false,
        corner_intersecting:
            vec![
                Point::new(0.0, 1.0),
                Point::new(1.0, 1.0),
                Point::new(1.0, 0.0)
            ],
        vec![
            Point::new(1.0, 1.0),
            Point::new(1.0, 2.0),
            Point::new(2.0, 1.0)
        ],
        true,
        overlapping:
            vec![
                Point::new(0.0, 1.0),
                Point::new(1.0, 1.0),
                Point::new(1.0, 0.0),
                Point::new(0.0, 0.0)
            ],
        vec![
            Point::new(0.5, 1.5),
            Point::new(1.5, 1.5),
            Point::new(1.5, 0.5),
            Point::new(0.5, 0.5)
        ],
        true,
        containing:
            vec![
                Point::new(0.0, 2.0),
                Point::new(2.0, 2.0),
                Point::new(2.0, 0.0),
                Point::new(0.0, 0.0)
            ],
        vec![
            Point::new(0.5, 1.5),
            Point::new(1.5, 1.5),
            Point::new(1.5, 0.5),
            Point::new(0.5, 0.5)
        ],
        true,
        contained:
            vec![
                Point::new(0.5, 1.5),
                Point::new(1.5, 1.5),
                Point::new(1.5, 0.5),
                Point::new(0.5, 0.5)
            ],
        vec![
            Point::new(0.0, 2.0),
            Point::new(2.0, 2.0),
            Point::new(2.0, 0.0),
            Point::new(0.0, 0.0)
        ],
        true,
    );

    #[test]
    fn basic_union() {
        let a = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);

        let b = Polygon::new(vec![
            Point::new(0.5, 0.5),
            Point::new(0.5, 1.5),
            Point::new(1.5, 1.5),
            Point::new(1.5, 0.5),
        ]);

        let expected = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(0.5, 1.0),
            Point::new(0.5, 1.5),
            Point::new(1.5, 1.5),
            Point::new(1.5, 0.5),
            Point::new(1.0, 0.5),
            Point::new(1.0, 0.0),
        ]);

        let result = a.union(&b);

        assert_eq!(result, expected);
    }
}
